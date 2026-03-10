//! Defines the [`RecycleList`] component for rendering large lists with virtualization.

use dioxus::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

/// The props for the [`RecycleList`] component.
#[derive(Props, Clone, PartialEq)]
pub struct RecycleListProps {
    /// The total number of items in the list.
    pub count: usize,
    /// The amount of render buffer (in estimated row counts) above and below the viewport.
    #[props(default = 8)]
    pub buffer: usize,
    /// Renders a single item by its absolute index.
    pub render_item: Callback<usize, Element>,
    /// Additional attributes to apply to the container element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// # RecycleList
///
/// The `RecycleList` component virtualizes a large list by rendering only the visible slice plus a
/// configurable buffer. It supports dynamic row heights and keeps total scroll height with a
/// virtual canvas.
///
/// Each rendered item receives `aria-setsize` and `aria-posinset` attributes for accessibility,
/// allowing screen readers to announce the total list size even though only a subset of items
/// is present in the DOM.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::recycle_list::RecycleList;
///
/// #[derive(Clone, PartialEq)]
/// struct Row {
///     title: String,
/// }
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         RecycleList {
///             count: 100,
///             buffer: 8,
///             render_item: move |idx: usize| rsx! {
///                 article { key: "{idx}", "Row {idx}" }
///             },
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`RecycleList`] component renders a container `div` with the class `recycle-list-container`.
/// All user-provided `attributes` are spread onto the container element.
#[component]
pub fn RecycleList(props: RecycleListProps) -> Element {
    let RecycleListProps {
        count,
        buffer,
        render_item,
        attributes,
    } = props;

    // Estimated item height used before the first measurement.
    let estimated_item_height: u32 = 100;
    let container_id = crate::use_unique_id();

    // Scroll position signal (relative to list top, in px).
    let mut scroll_top = use_signal(|| 0u32);
    let mut viewport_height = use_signal(|| estimated_item_height.saturating_mul(8));
    let mut measured_heights = use_signal(|| vec![estimated_item_height; count]);
    let mut measured_flags = use_signal(|| vec![false; count]);

    // Buffer height measurements during active scrolling to prevent scrollHeight jumps.
    let mut is_scrolling = use_signal(|| false);
    let mut pending_heights: Signal<HashMap<usize, u32>> = use_signal(HashMap::new);

    // Keep height cache length aligned with current items.
    if measured_heights.read().len() != count {
        measured_heights.with_mut(|heights| heights.resize(count, estimated_item_height));
        measured_flags.with_mut(|flags| flags.resize(count, false));
    }

    // Subscribe to the container scroll via a JS eval bridge.
    // We keep the list in a single scroll mode and coalesce updates into
    // requestAnimationFrame so dragging the native scrollbar stays responsive.
    use_effect(move || {
        let script = r#"
            const container = document.getElementById(await dioxus.recv());
            if (!container) return;

            let idleTimer = null;
            let frame = null;

            function publish(force = false) {
                frame = null;
                const scroll = container.scrollTop;
                const viewport = Math.min(container.clientHeight, window.innerHeight) || 600;
                dioxus.send(JSON.stringify([Math.round(scroll), viewport]));
            }

            function scheduleUpdate() {
                if (frame === null) {
                    frame = requestAnimationFrame(() => publish(false));
                }

                if (idleTimer) clearTimeout(idleTimer);
                idleTimer = setTimeout(() => {
                    publish(true);
                    dioxus.send("idle");
                }, 180);
            }

            scheduleUpdate();
            container.addEventListener("scroll", scheduleUpdate, { passive: true });
            window.addEventListener("scroll", scheduleUpdate, { passive: true });
            window.addEventListener("resize", scheduleUpdate, { passive: true });
            await dioxus.recv();
            if (frame !== null) cancelAnimationFrame(frame);
            if (idleTimer) clearTimeout(idleTimer);
            container.removeEventListener("scroll", scheduleUpdate);
            window.removeEventListener("scroll", scheduleUpdate);
            window.removeEventListener("resize", scheduleUpdate);
            "#;
        let mut eval = document::eval(script);

        let _ = eval.send(container_id.peek().clone());

        spawn(async move {
            while let Ok(msg) = eval.recv::<String>().await {
                if msg.trim() == "idle" {
                    is_scrolling.set(false);
                    // Flush pending height measurements.
                    let pending = pending_heights.with_mut(std::mem::take);
                    if !pending.is_empty() {
                        let current_scroll = *scroll_top.peek();
                        let (new_heights, compensated_scroll) = {
                            let heights = measured_heights.read();
                            let flags = measured_flags.read();
                            let mut old_prefix = Vec::with_capacity(heights.len() + 1);
                            old_prefix.push(0u32);
                            for &height in heights.iter() {
                                let next = old_prefix
                                    .last()
                                    .copied()
                                    .unwrap_or(0)
                                    .saturating_add(height.max(1));
                                old_prefix.push(next);
                            }

                            let max_scroll =
                                old_prefix.last().copied().unwrap_or(0).saturating_sub(1);
                            let clamped_scroll = current_scroll.min(max_scroll);
                            let anchor_idx = old_prefix
                                .partition_point(|&acc| acc <= clamped_scroll)
                                .saturating_sub(1);
                            let anchor_idx = anchor_idx.min(heights.len().saturating_sub(1));
                            let anchor_offset =
                                clamped_scroll.saturating_sub(old_prefix[anchor_idx]);

                            let mut next_heights = heights.clone();
                            let mut next_flags = flags.clone();
                            for (&idx, &h) in &pending {
                                if idx < next_heights.len() && next_heights[idx] != h {
                                    next_heights[idx] = h;
                                }
                                if idx < next_flags.len() {
                                    next_flags[idx] = true;
                                }
                            }

                            let measured_count =
                                next_flags.iter().filter(|&&measured| measured).count();
                            let measured_total_height: u64 = next_heights
                                .iter()
                                .zip(next_flags.iter())
                                .filter_map(|(height, measured)| {
                                    measured.then_some(u64::from(*height))
                                })
                                .sum();
                            let adaptive_estimate = if measured_count > 0 {
                                (measured_total_height / measured_count as u64)
                                    .clamp(1, u64::from(u32::MAX))
                                    as u32
                            } else {
                                estimated_item_height
                            };

                            for (idx, measured) in next_flags.iter().copied().enumerate() {
                                if !measured && idx < next_heights.len() {
                                    next_heights[idx] = adaptive_estimate;
                                }
                            }

                            let mut new_prefix = Vec::with_capacity(next_heights.len() + 1);
                            new_prefix.push(0u32);
                            for &height in next_heights.iter() {
                                let next = new_prefix
                                    .last()
                                    .copied()
                                    .unwrap_or(0)
                                    .saturating_add(height.max(1));
                                new_prefix.push(next);
                            }

                            let compensated_scroll = if next_heights.is_empty() {
                                0
                            } else {
                                let new_anchor_height = next_heights
                                    .get(anchor_idx)
                                    .copied()
                                    .unwrap_or(estimated_item_height)
                                    .max(1);
                                new_prefix[anchor_idx].saturating_add(
                                    anchor_offset.min(new_anchor_height.saturating_sub(1)),
                                )
                            };

                            (next_heights, compensated_scroll)
                        };

                        measured_heights.set(new_heights);
                        measured_flags.with_mut(|flags| {
                            for &idx in pending.keys() {
                                if idx < flags.len() {
                                    flags[idx] = true;
                                }
                            }
                        });

                        if compensated_scroll != current_scroll {
                            scroll_top.set(compensated_scroll);
                            let container_id = container_id.peek().clone();
                            spawn(async move {
                                sync_container_scroll(container_id, compensated_scroll).await;
                            });
                        }
                    }
                } else if let Some((s, v)) = parse_scroll_msg(&msg) {
                    is_scrolling.set(true);
                    if s != *scroll_top.peek() {
                        scroll_top.set(s);
                    }
                    if v != *viewport_height.peek() {
                        viewport_height.set(v);
                    }
                }
            }
        });
    });

    // Calculate the render window from scroll position.
    let current_scroll = *scroll_top.read();
    let viewport_px = (*viewport_height.read()).max(estimated_item_height);
    let buffer_px = (buffer as u32).saturating_mul(estimated_item_height);

    // Rebuild prefix sums when measured heights change.
    let prefix_and_total = use_memo({
        let measured_heights = measured_heights;
        move || {
            let heights = measured_heights.read();
            let mut prefix: Vec<u32> = Vec::with_capacity(heights.len() + 1);
            prefix.push(0);
            for height in heights.iter() {
                let next = prefix
                    .last()
                    .copied()
                    .unwrap_or(0)
                    .saturating_add((*height).max(1));
                prefix.push(next);
            }
            let total_height = *prefix.last().unwrap_or(&0);
            (Arc::new(prefix), total_height)
        }
    });

    // Resolve visible range from prefix sums.
    let (prefix, total_height) = prefix_and_total();
    let prefix: &[u32] = prefix.as_ref();

    let (render_start, mut end_idx) = if count == 0 {
        (0, 0)
    } else {
        let item_at = |y: u32| prefix.partition_point(|&acc| acc <= y).saturating_sub(1);
        let clamped_scroll = current_scroll.min(total_height.saturating_sub(1));
        let render_start = item_at(clamped_scroll.saturating_sub(buffer_px));

        let end_target = clamped_scroll
            .saturating_add(viewport_px)
            .saturating_add(buffer_px);
        let end_idx = prefix.partition_point(|&acc| acc < end_target).min(count);

        (render_start, end_idx)
    };

    if count > 0 && end_idx <= render_start {
        end_idx = (render_start + 1).min(count);
    }

    let top_offset = prefix[render_start];
    let canvas_height = total_height.max(viewport_px);

    let set_size = count.to_string();

    rsx! {
        div {
            id: container_id,
            class: "recycle-list-container",
            role: "list",
            tabindex: "0",
            ..attributes,

            div {
                style: "position: relative; height:{canvas_height}px; width: 100%;",
                div {
                    style: "position: absolute; inset: 0 auto auto 0; width: 100%; transform: translateY({top_offset}px); will-change: transform;",
                    {(render_start..end_idx).map(|idx| {
                        let measured_heights_for_item = measured_heights;
                        let measured_flags_for_item = measured_flags;
                        let set_size = set_size.clone();

                        rsx! {
                            div {
                                key: "{idx}",
                                role: "listitem",
                                "aria-setsize": set_size,
                                "aria-posinset": "{idx + 1}",
                                onmounted: move |event: Event<MountedData>| {
                                    let mut measured_heights_for_item = measured_heights_for_item;
                                    let mut measured_flags_for_item = measured_flags_for_item;
                                    let already_measured = measured_flags_for_item
                                        .read()
                                        .get(idx)
                                        .copied()
                                        .unwrap_or(false);

                                    if already_measured {
                                        return;
                                    }

                                    spawn(async move {
                                        let rect = event.get_client_rect().await.unwrap_or_default();
                                        let measured = rect.height().max(1.0).round() as u32;
                                        if *is_scrolling.peek() {
                                            // Buffer during scroll to prevent scrollHeight jumps.
                                            pending_heights.with_mut(|p| {
                                                p.insert(idx, measured);
                                            });
                                        } else {
                                            measured_heights_for_item.with_mut(|heights| {
                                                if idx < heights.len() && heights[idx] != measured {
                                                    heights[idx] = measured;
                                                }
                                            });
                                            measured_flags_for_item.with_mut(|flags| {
                                                if idx < flags.len() {
                                                    flags[idx] = true;
                                                }
                                            });
                                        }
                                    });
                                },
                                {render_item(idx)}
                            }
                        }
                    })}
                }
            }
        }
    }
}

/// Parse a `"[scrollTop, viewportHeight]"` JSON array from the JS bridge.
fn parse_scroll_msg(msg: &str) -> Option<(u32, u32)> {
    let msg = msg.trim().trim_start_matches('[').trim_end_matches(']');
    let mut parts = msg.split(',');
    let s = parts.next()?.trim().parse::<f64>().ok()?;
    let v = parts.next()?.trim().parse::<f64>().ok()?;
    Some((s.max(0.0).round() as u32, v.max(1.0).round() as u32))
}

async fn sync_container_scroll(container_id: String, scroll_top: u32) {
    let eval = document::eval(
        r#"
        const id = await dioxus.recv();
        const targetScroll = await dioxus.recv();
        const container = document.getElementById(id);
        if (container) {
            container.scrollTop = targetScroll;
        }
        "#,
    );
    let _ = eval.send(container_id);
    let _ = eval.send(scroll_top);
}
