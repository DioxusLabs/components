//! Framework-agnostic virtual list implementation.
//!
//! This module provides a Rust port of TanStack Virtual's virtual-core,
//! containing all the algorithms needed for efficient list virtualization:
//!
//! - Computing item positions from measured or estimated sizes
//! - Calculating the visible range using binary search
//! - Handling scroll position corrections when items resize
//! - Managing programmatic scroll operations (scroll-to-index)
//!
//! # Architecture
//!
//! The [`Virtualizer`] struct is framework-agnostic and contains pure algorithms.
//! Framework adapters (like the `RecycleList` Dioxus component) wrap `Virtualizer`
//! and handle:
//! - DOM observation (scroll events, resize events)
//! - Element measurement
//! - Applying scroll corrections
//! - Rendering virtual items
//!
//! # Example
//!
//! ```rust
//! use dioxus_primitives::r#virtual::{Virtualizer, VirtualizerOptions, Rect};
//!
//! // Create a virtualizer for 1000 items with estimated height of 50px
//! let options = VirtualizerOptions::new(1000, |_| 50, |i| i)
//!     .overscan(4);
//!
//! let mut virtualizer = Virtualizer::new(options);
//!
//! // Set the viewport size
//! virtualizer.set_viewport_size(Rect::new(800, 600));
//!
//! // When the user scrolls
//! virtualizer.set_scroll_offset(500, true);
//!
//! // Get items to render
//! let items = virtualizer.get_virtual_items();
//!
//! // When an item is measured (e.g., from ResizeObserver)
//! if let Some(scroll_adjustment) = virtualizer.resize_item(10, 75) {
//!     // Apply scroll_adjustment to the DOM to prevent jumping
//!     // container.scrollTop += scroll_adjustment;
//! }
//! ```
//!
//! # Scroll Position Correction
//!
//! The key feature that prevents scroll jumping is the [`Virtualizer::resize_item`]
//! method. When an item's actual size differs from its estimated size:
//!
//! - If the item is **above** the viewport, we adjust the scroll position by the
//!   size delta so the visible content stays in place.
//! - If the item is **below** the viewport, no adjustment is needed since it
//!   doesn't affect the visible content.
//!
//! This matches TanStack Virtual's approach and is the key insight for smooth
//! virtualization with dynamic heights.

mod types;
mod utils;
mod virtualizer;

pub use types::{
    Key, Range, Rect, ScrollAlignment, ScrollBehavior, ScrollDirection, ScrollState,
    ScrollToOptions, VirtualItem,
};

pub use utils::{approx_equal, default_range_extractor, find_nearest_binary_search};

pub use virtualizer::{Virtualizer, VirtualizerOptions};
