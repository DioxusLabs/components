//! Core types for the virtual list implementation.

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
