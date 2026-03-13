//! Defines the [`VirtualList`] component for rendering large lists with virtualization.

use dioxus::prelude::*;
use serde::Deserialize;
use std::cell::RefCell;
use std::rc::Rc;

use crate::r#virtual::{Virtualizer, VirtualizerOptions};

/// The props for the [`VirtualList`] component.
#[derive(Props, Clone, PartialEq)]
pub struct VirtualListProps {
    /// The total number of items in the list.
    pub count: usize,
    /// The amount of render buffer (in estimated row counts) above and below the viewport.
    #[props(default = 8)]
    pub buffer: usize,
    /// Estimates the height of an item by index (used before measurement).
    /// For best scrollbar stability, return values close to actual heights.
    /// If not provided, uses adaptive estimation based on measured items.
    pub estimate_size: Option<Callback<usize, u32>>,
    /// Renders a single item by its absolute index.
    pub render_item: Callback<usize, Element>,
    /// Additional attributes to apply to the container element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// # VirtualList
///
/// The `VirtualList` component virtualizes a large list by rendering only the visible slice plus a
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
/// use dioxus_primitives::virtual_list::VirtualList;
///
/// #[derive(Clone, PartialEq)]
/// struct Row {
///     title: String,
/// }
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         VirtualList {
///             count: 100,
///             buffer: 8,
///             // Optional: estimate height per item for smoother scrolling
///             // If omitted, uses adaptive estimation based on measured items
///             estimate_size: |_idx| 48,
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
/// The [`VirtualList`] component renders a container `div` with the class `virtual-list-container`.
/// All user-provided `attributes` are spread onto the container element.
#[component]
pub fn VirtualList(props: VirtualListProps) -> Element {
    let VirtualListProps {
        count,
        buffer,
        estimate_size,
        render_item,
        attributes,
    } = props;

    let container_id = crate::use_unique_id();

    // Create virtualizer wrapped in Rc<RefCell> for shared mutable access
    let virtualizer: Signal<Rc<RefCell<Virtualizer<_, _>>>> = use_signal(|| {
        let estimate_fn = move |i| {
            estimate_size.map(|cb| cb(i)).unwrap_or(100) // Default 100px, will adapt
        };
        let options = VirtualizerOptions::new(count, estimate_fn, |i| i)
            .overscan(buffer)
            .use_adaptive_estimation(estimate_size.is_none());
        Rc::new(RefCell::new(Virtualizer::new(options)))
    });

    // Track scroll state
    let mut scroll_offset = use_signal(|| 0u32);
    let mut viewport_height = use_signal(|| 600);
    let mut is_scrolling = use_signal(|| false);

    // Update virtualizer when count changes
    use_effect(move || {
        virtualizer.peek().borrow_mut().set_count(count);
    });

    // Subscribe to scroll events via JS bridge
    use_effect(move || {
        let script = r#"
            const container = document.getElementById(await dioxus.recv());
            if (!container) return;

            let scrollEndTimer = null;
            let lastOffset = null;

            function publish(isScrolling) {
                const scroll = Math.round(container.scrollTop);
                // Deduplicate: don't send if offset hasn't changed
                if (!isScrolling && scroll === lastOffset) return;
                lastOffset = scroll;
                const viewport = Math.min(container.clientHeight, window.innerHeight) || 600;
                dioxus.send({
                    offset: scroll,
                    viewport: viewport,
                    isScrolling: isScrolling
                });
            }

            function onScroll() {
                // Clear any pending scroll-end detection
                if (scrollEndTimer !== null) {
                    clearTimeout(scrollEndTimer);
                }

                // Send scroll event immediately (no RAF batching)
                // This ensures Rust receives the event before the next render
                publish(true);

                // Debounce scroll-end detection (150ms after last scroll event)
                scrollEndTimer = setTimeout(() => {
                    scrollEndTimer = null;
                    publish(false);
                }, 150);
            }

            // Initial publish
            publish(false);

            container.addEventListener("scroll", onScroll, { passive: true });
            window.addEventListener("resize", () => publish(false), { passive: true });

            await dioxus.recv();
            if (scrollEndTimer !== null) clearTimeout(scrollEndTimer);
            container.removeEventListener("scroll", onScroll);
        "#;
        let mut eval = document::eval(script);
        let _ = eval.send(container_id.peek().clone());

        spawn(async move {
            while let Ok(scroll_msg) = eval.recv::<ScrollMsg>().await {
                let scrolling = scroll_msg.is_scrolling;

                // Update virtualizer state and get any scroll correction
                let correction = {
                    let v_rc = virtualizer.peek();
                    let mut v = v_rc.borrow_mut();
                    let correction = v.set_scroll_offset(scroll_msg.offset, scrolling);
                    v.set_viewport_size(scroll_msg.viewport);
                    correction
                };

                // Update is_scrolling FIRST so re-renders see correct state
                if scrolling != *is_scrolling.peek() {
                    is_scrolling.set(scrolling);
                }

                // Apply scroll correction when scrolling stops (to compensate for
                // height changes that occurred during scrolling)
                if let Some(delta) = correction {
                    let new_scroll = (scroll_msg.offset as i32 + delta).max(0) as u32;
                    sync_container_scroll(container_id.peek().clone(), new_scroll).await;
                    scroll_offset.set(new_scroll);
                } else {
                    // Update scroll offset to trigger re-render
                    if scroll_msg.offset != *scroll_offset.peek() {
                        scroll_offset.set(scroll_msg.offset);
                    }
                }
                if scroll_msg.viewport != *viewport_height.peek() {
                    viewport_height.set(scroll_msg.viewport);
                }
            }
        });
    });

    // Read scroll state to establish reactive dependency
    let current_scroll = *scroll_offset.read();
    let current_viewport = *viewport_height.read();
    // Read is_scrolling to trigger re-render on scroll-end (for unfreezing total_size)
    let _ = *is_scrolling.read();

    // Get computed values from virtualizer
    // Use sync_scroll_offset instead of set_scroll_offset to update the position
    // without affecting is_scrolling state - the event handler manages scroll state
    // transitions and stable_total_size freezing.
    let (virtual_items, total_height) = {
        let v_rc = virtualizer.peek();
        let mut v = v_rc.borrow_mut();
        v.sync_scroll_offset(current_scroll);
        v.set_viewport_size(current_viewport);
        let items = v.get_virtual_items();
        let total = v.get_total_size();
        (items, total)
    };

    // Calculate the top offset for the content wrapper
    let top_offset = virtual_items.first().map(|item| item.start).unwrap_or(0);
    let canvas_height = total_height.max(current_viewport);
    let set_size = count.to_string();

    rsx! {
        div {
            id: container_id,
            class: "virtual-list-container",
            role: "list",
            tabindex: "0",
            ..attributes,

            div {
                style: "position: relative; height:{canvas_height}px; width: 100%;",
                div {
                    style: "position: absolute; inset: 0 auto auto 0; width: 100%; transform: translateY({top_offset}px); will-change: transform;",
                    {virtual_items.iter().map(|item| {
                        let idx = item.index;
                        let scroll_offset = scroll_offset;
                        let set_size = set_size.clone();

                        rsx! {
                            div {
                                key: "{idx}",
                                role: "listitem",
                                "data-virtual-index": "{idx}",
                                "aria-setsize": set_size,
                                "aria-posinset": "{idx + 1}",
                                onresize: move |event: Event<ResizeData>| {
                                    let rect = event.data().get_content_box_size().unwrap_or_default();
                                    let measured = rect.height.max(1.0).round() as u32;

                                    // Apply measurement and get scroll adjustment
                                    let adjustment = {
                                        let v_rc = virtualizer.peek();
                                        let mut v = v_rc.borrow_mut();
                                        v.resize_item(idx, measured)
                                    };

                                    // Apply scroll correction immediately if needed
                                    if let Some(delta) = adjustment {
                                        let current = *scroll_offset.peek();
                                        let new_scroll = (current as i32 + delta).max(0) as u32;
                                        spawn(async move {
                                            sync_container_scroll(
                                                container_id.peek().clone(),
                                                new_scroll,
                                            ).await;
                                        });
                                    }
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

/// Parsed scroll message from JS bridge.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ScrollMsg {
    offset: u32,
    viewport: u32,
    is_scrolling: bool,
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
