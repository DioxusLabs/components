//! Core types for the virtual list implementation.
//!
//! These types are framework-agnostic and mirror TanStack Virtual's virtual-core.

/// A unique key for identifying items in the virtualizer.
pub type Key = usize;

/// Direction of scroll movement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollDirection {
    /// Scrolling towards higher offsets (down or right).
    Forward,
    /// Scrolling towards lower offsets (up or left).
    Backward,
}

/// Alignment for programmatic scroll operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScrollAlignment {
    /// Align item to the start of the viewport.
    Start,
    /// Align item to the center of the viewport.
    Center,
    /// Align item to the end of the viewport.
    End,
    /// Automatically determine alignment based on current visibility.
    #[default]
    Auto,
}

/// Behavior for programmatic scroll operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScrollBehavior {
    /// Let the browser decide (usually instant).
    #[default]
    Auto,
    /// Smooth animated scrolling.
    Smooth,
    /// Instant jump to position.
    Instant,
}

/// A single virtualized item with computed position.
#[derive(Debug, Clone, PartialEq)]
pub struct VirtualItem {
    /// Unique key for this item (used for caching measurements).
    pub key: Key,
    /// Original index in the data source.
    pub index: usize,
    /// Start offset in pixels from the beginning of the scroll container.
    pub start: u32,
    /// End offset in pixels (start + size).
    pub end: u32,
    /// Size of the item in pixels (height for vertical, width for horizontal).
    pub size: u32,
    /// Lane assignment for multi-column/masonry layouts.
    pub lane: usize,
}

impl VirtualItem {
    /// Creates a new VirtualItem.
    pub fn new(key: Key, index: usize, start: u32, size: u32, lane: usize) -> Self {
        Self {
            key,
            index,
            start,
            end: start + size,
            size,
            lane,
        }
    }
}

/// The visible range of items in the virtualizer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    /// Index of the first visible item.
    pub start_index: usize,
    /// Index of the last visible item (inclusive).
    pub end_index: usize,
}

impl Range {
    /// Creates a new Range.
    pub fn new(start_index: usize, end_index: usize) -> Self {
        Self {
            start_index,
            end_index,
        }
    }

    /// Returns the number of items in this range.
    pub fn len(&self) -> usize {
        if self.end_index >= self.start_index {
            self.end_index - self.start_index + 1
        } else {
            0
        }
    }

    /// Returns true if the range is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Viewport dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect {
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
}

impl Rect {
    /// Creates a new Rect.
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Returns the size in the main axis direction.
    pub fn size(&self, horizontal: bool) -> u32 {
        if horizontal {
            self.width
        } else {
            self.height
        }
    }
}

/// State for tracking scroll reconciliation during programmatic scrolling.
///
/// When `scroll_to_index` or `scroll_to_offset` is called, this state tracks
/// the progress of reaching the target position, handling cases where item
/// measurements change during the scroll.
#[derive(Debug, Clone)]
pub struct ScrollState {
    /// Target item index (if scrolling to an index).
    pub index: Option<usize>,
    /// Desired alignment at the target.
    pub align: ScrollAlignment,
    /// Scroll behavior (smooth, instant, auto).
    pub behavior: ScrollBehavior,
    /// Timestamp when scroll operation started (milliseconds).
    pub started_at_ms: u64,
    /// Last computed target offset.
    pub last_target_offset: u32,
    /// Number of consecutive frames where scroll position was stable.
    pub stable_frames: u8,
}

impl ScrollState {
    /// Maximum time to attempt reconciliation before giving up (5 seconds).
    pub const MAX_RECONCILE_MS: u64 = 5000;

    /// Number of stable frames required before considering scroll complete.
    pub const STABLE_FRAMES_REQUIRED: u8 = 1;

    /// Creates a new ScrollState for scrolling to an index.
    pub fn to_index(
        index: usize,
        align: ScrollAlignment,
        behavior: ScrollBehavior,
        now_ms: u64,
        initial_offset: u32,
    ) -> Self {
        Self {
            index: Some(index),
            align,
            behavior,
            started_at_ms: now_ms,
            last_target_offset: initial_offset,
            stable_frames: 0,
        }
    }

    /// Creates a new ScrollState for scrolling to an offset.
    pub fn to_offset(
        offset: u32,
        align: ScrollAlignment,
        behavior: ScrollBehavior,
        now_ms: u64,
    ) -> Self {
        Self {
            index: None,
            align,
            behavior,
            started_at_ms: now_ms,
            last_target_offset: offset,
            stable_frames: 0,
        }
    }

    /// Returns true if the reconciliation has exceeded the maximum time.
    pub fn is_expired(&self, now_ms: u64) -> bool {
        now_ms.saturating_sub(self.started_at_ms) > Self::MAX_RECONCILE_MS
    }

    /// Returns true if scroll has been stable for enough frames.
    pub fn is_stable(&self) -> bool {
        self.stable_frames >= Self::STABLE_FRAMES_REQUIRED
    }
}

/// Options for configuring scroll-to operations.
#[derive(Debug, Clone, Copy, Default)]
pub struct ScrollToOptions {
    /// Alignment of the target item/offset in the viewport.
    pub align: ScrollAlignment,
    /// Scroll behavior (smooth, instant, auto).
    pub behavior: ScrollBehavior,
}

impl ScrollToOptions {
    /// Creates new ScrollToOptions with the specified alignment and behavior.
    pub fn new(align: ScrollAlignment, behavior: ScrollBehavior) -> Self {
        Self { align, behavior }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtual_item_new() {
        let item = VirtualItem::new(5, 5, 100, 50, 0);
        assert_eq!(item.key, 5);
        assert_eq!(item.index, 5);
        assert_eq!(item.start, 100);
        assert_eq!(item.end, 150);
        assert_eq!(item.size, 50);
        assert_eq!(item.lane, 0);
    }

    #[test]
    fn test_range_len() {
        let range = Range::new(5, 10);
        assert_eq!(range.len(), 6);

        let empty_range = Range::new(10, 5);
        assert_eq!(empty_range.len(), 0);
        assert!(empty_range.is_empty());
    }

    #[test]
    fn test_rect_size() {
        let rect = Rect::new(800, 600);
        assert_eq!(rect.size(true), 800); // horizontal
        assert_eq!(rect.size(false), 600); // vertical
    }

    #[test]
    fn test_scroll_state_expiry() {
        let state = ScrollState::to_index(0, ScrollAlignment::Start, ScrollBehavior::Auto, 1000, 0);
        assert!(!state.is_expired(2000));
        assert!(!state.is_expired(5999));
        assert!(state.is_expired(6001));
    }
}
