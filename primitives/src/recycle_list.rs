//! Defines the [`RecycleList`] component for rendering large lists with virtualization.

use dioxus::prelude::*;
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
/// configurable buffer. It supports dynamic row heights and keeps total scroll height with spacers.
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
///     let rows: Vec<Row> = (0..2000)
///         .map(|i| Row {
///             title: format!("Row {i}"),
///         })
///         .collect();
///
///     rsx! {
///         RecycleList {
///             count: rows.len(),
///             buffer: 8,
///             render_item: move |idx: usize| rsx! {
///                 article { key: "{idx}", "{rows[idx].title}" }
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

    // Keep height cache length aligned with current items.
    if measured_heights.read().len() != count {
        measured_heights.with_mut(|heights| heights.resize(count, estimated_item_height));
    }

    // Subscribe to scroll events via a JS eval bridge.
    // Listens to both container and window scroll, dynamically detecting the
    // correct scroll mode on each event (handles async CSS loading).
    use_effect(move || {
        let mut eval = document::eval(
            r#"
            const container = document.getElementById(await dioxus.recv());
            if (!container) return;

            const winScrollY = () => window.scrollY || window.pageYOffset || 0;
            const initRect = container.getBoundingClientRect();
            const containerPageTop = initRect.top + winScrollY();

            function sendUpdate() {
                const isContainerScroll =
                    container.scrollHeight > container.clientHeight + 1 &&
                    container.clientHeight > 0;

                let scroll, viewport;
                if (isContainerScroll) {
                    scroll = container.scrollTop;
                    viewport = container.clientHeight;
                } else {
                    scroll = Math.max(0, winScrollY() - containerPageTop);
                    viewport = window.innerHeight || 600;
                }
                dioxus.send(JSON.stringify([Math.round(scroll), viewport]));
            }

            sendUpdate();
            container.addEventListener("scroll", sendUpdate, { passive: true });
            window.addEventListener("scroll", sendUpdate, { passive: true });
            window.addEventListener("resize", sendUpdate, { passive: true });
            await dioxus.recv();
            container.removeEventListener("scroll", sendUpdate);
            window.removeEventListener("scroll", sendUpdate);
            window.removeEventListener("resize", sendUpdate);
            "#,
        );

        let _ = eval.send(container_id.peek().clone());

        spawn(async move {
            while let Ok(msg) = eval.recv::<String>().await {
                if let Some((s, v)) = parse_scroll_msg(&msg) {
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

    let top_spacer = prefix[render_start];
    let bottom_spacer = total_height.saturating_sub(prefix[end_idx]);

    let set_size = count.to_string();

    rsx! {
        div {
            id: container_id,
            class: "recycle-list-container",
            role: "list",
            tabindex: "0",
            ..attributes,

            div { style: "height:{top_spacer}px; width:1px;" }

            {(render_start..end_idx).map(|idx| {
                let measured_heights_for_item = measured_heights;
                let set_size = set_size.clone();

                rsx! {
                    div {
                        key: "{idx}",
                        role: "listitem",
                        "aria-setsize": set_size,
                        "aria-posinset": "{idx + 1}",
                        onmounted: move |event: Event<MountedData>| {
                            let mut measured_heights_for_item = measured_heights_for_item;
                            spawn(async move {
                                let rect = event.get_client_rect().await.unwrap_or_default();
                                let measured = rect.height().max(1.0).round() as u32;
                                measured_heights_for_item
                                    .with_mut(|heights| {
                                        if idx < heights.len() && heights[idx] != measured {
                                            heights[idx] = measured;
                                        }
                                    });
                            });
                        },
                        {render_item(idx)}
                    }
                }
            })}

            div { style: "height:{bottom_spacer}px; width:1px;" }
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
