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
                        let cid = container_id.peek().clone();
                        let current_scroll = *scroll_top.peek();
                        let dom_anchor = capture_visible_anchor(cid.clone()).await;
                        let (new_heights, new_flags, compensated_scroll) = {
                            let heights = measured_heights.read();
                            let flags = measured_flags.read();
                            apply_pending_measurements(
                                &heights,
                                &flags,
                                &pending,
                                current_scroll,
                                estimated_item_height,
                            )
                        };

                        measured_heights.set(new_heights);
                        measured_flags.set(new_flags);

                        let restored_scroll = if let Some((anchor_idx, anchor_offset)) = dom_anchor
                        {
                            restore_visible_anchor(cid.clone(), anchor_idx, anchor_offset).await
                        } else {
                            None
                        };
                        let final_scroll = restored_scroll.unwrap_or(compensated_scroll);

                        if final_scroll != current_scroll {
                            scroll_top.set(final_scroll);
                            if restored_scroll.is_none() {
                                sync_container_scroll(cid, final_scroll).await;
                            }
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
        let measured_flags = measured_flags;
        move || {
            let heights = measured_heights.read();
            let flags = measured_flags.read();
            let adaptive = estimate_unmeasured_height(&heights, &flags, estimated_item_height);
            let prefix = build_prefix_sums_adaptive(&heights, &flags, adaptive);
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
                                "data-recycle-index": "{idx}",
                                "aria-setsize": set_size,
                                "aria-posinset": "{idx + 1}",
                                onmounted: move |event: Event<MountedData>| {
                                    let mut measured_heights_for_item = measured_heights_for_item;
                                    let mut measured_flags_for_item = measured_flags_for_item;
                                    let already_measured = measured_flags_for_item
                                        .peek()
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

fn apply_pending_measurements(
    heights: &[u32],
    flags: &[bool],
    pending: &HashMap<usize, u32>,
    current_scroll: u32,
    estimated_item_height: u32,
) -> (Vec<u32>, Vec<bool>, u32) {
    // Build old prefix using adaptive estimates (matching what use_memo renders).
    let old_adaptive = estimate_unmeasured_height(heights, flags, estimated_item_height);
    let old_prefix = build_prefix_sums_adaptive(heights, flags, old_adaptive);
    let (anchor_idx, anchor_offset) = find_scroll_anchor(&old_prefix, current_scroll);

    let mut next_heights = heights.to_vec();
    let mut next_flags = flags.to_vec();
    for (&idx, &height) in pending {
        if idx < next_heights.len() {
            next_heights[idx] = height;
        }
        if idx < next_flags.len() {
            next_flags[idx] = true;
        }
    }

    // Build new prefix using adaptive estimates (matching what use_memo will render).
    let new_adaptive =
        estimate_unmeasured_height(&next_heights, &next_flags, estimated_item_height);
    let new_prefix = build_prefix_sums_adaptive(&next_heights, &next_flags, new_adaptive);
    let compensated_scroll = if next_heights.is_empty() {
        0
    } else {
        let anchor_height = if next_flags.get(anchor_idx).copied().unwrap_or(false) {
            next_heights[anchor_idx]
        } else {
            new_adaptive
        }
        .max(1);
        new_prefix[anchor_idx].saturating_add(anchor_offset.min(anchor_height.saturating_sub(1)))
    };

    (next_heights, next_flags, compensated_scroll)
}

/// Build prefix sums substituting `adaptive` for unmeasured items.
/// Avoids allocating an intermediate effective-heights Vec.
fn build_prefix_sums_adaptive(heights: &[u32], flags: &[bool], adaptive: u32) -> Vec<u32> {
    let mut prefix = Vec::with_capacity(heights.len() + 1);
    prefix.push(0u32);
    let mut acc = 0u32;
    for (i, &height) in heights.iter().enumerate() {
        let h = if flags.get(i).copied().unwrap_or(false) {
            height
        } else {
            adaptive
        };
        acc = acc.saturating_add(h.max(1));
        prefix.push(acc);
    }
    prefix
}

fn find_scroll_anchor(prefix: &[u32], current_scroll: u32) -> (usize, u32) {
    if prefix.len() <= 1 {
        return (0, 0);
    }

    let max_scroll = prefix.last().copied().unwrap_or(0).saturating_sub(1);
    let clamped_scroll = current_scroll.min(max_scroll);
    let anchor_idx = prefix
        .partition_point(|&acc| acc <= clamped_scroll)
        .saturating_sub(1)
        .min(prefix.len().saturating_sub(2));
    let anchor_offset = clamped_scroll.saturating_sub(prefix[anchor_idx]);

    (anchor_idx, anchor_offset)
}

fn estimate_unmeasured_height(heights: &[u32], flags: &[bool], fallback: u32) -> u32 {
    let measured_count = flags.iter().filter(|&&measured| measured).count();
    if measured_count == 0 {
        return fallback;
    }

    let measured_total: u64 = heights
        .iter()
        .zip(flags.iter())
        .filter_map(|(height, measured)| measured.then_some(u64::from(*height)))
        .sum();

    (measured_total / measured_count as u64).clamp(1, u64::from(u32::MAX)) as u32
}

fn parse_anchor_msg(msg: &str) -> Option<(usize, f64)> {
    let (idx, offset) = msg.trim().split_once(',')?;
    Some((idx.trim().parse().ok()?, offset.trim().parse().ok()?))
}

async fn capture_visible_anchor(container_id: String) -> Option<(usize, f64)> {
    let mut eval = document::eval(
        r#"
        const id = await dioxus.recv();
        const container = document.getElementById(id);
        if (!container) {
            dioxus.send("missing");
            return;
        }

        const containerRect = container.getBoundingClientRect();
        const items = container.querySelectorAll("[data-recycle-index]");
        for (const item of items) {
            const rect = item.getBoundingClientRect();
            if (rect.bottom > containerRect.top && rect.top < containerRect.bottom) {
                const idx = item.getAttribute("data-recycle-index");
                const offset = rect.top - containerRect.top;
                dioxus.send(`${idx},${offset}`);
                return;
            }
        }

        dioxus.send("missing");
        "#,
    );
    let _ = eval.send(container_id);
    match eval.recv::<String>().await.ok() {
        Some(msg) if msg.trim() != "missing" => parse_anchor_msg(&msg),
        _ => None,
    }
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

async fn restore_visible_anchor(
    container_id: String,
    anchor_idx: usize,
    anchor_offset: f64,
) -> Option<u32> {
    let mut eval = document::eval(
        r#"
        const id = await dioxus.recv();
        const anchorIdx = await dioxus.recv();
        const anchorOffset = await dioxus.recv();

        await new Promise(resolve => requestAnimationFrame(resolve));

        const container = document.getElementById(id);
        if (!container) {
            dioxus.send("missing");
            return;
        }

        const item = container.querySelector(`[data-recycle-index="${anchorIdx}"]`);
        if (!item) {
            dioxus.send("missing");
            return;
        }

        const containerRect = container.getBoundingClientRect();
        const itemRect = item.getBoundingClientRect();
        const targetTop = containerRect.top + anchorOffset;
        const delta = itemRect.top - targetTop;
        container.scrollTop += delta;
        dioxus.send(String(Math.round(container.scrollTop)));
        "#,
    );
    let _ = eval.send(container_id);
    let _ = eval.send(anchor_idx);
    let _ = eval.send(anchor_offset);
    match eval.recv::<String>().await.ok() {
        Some(msg) if msg.trim() != "missing" => msg.trim().parse::<u32>().ok(),
        _ => None,
    }
}
