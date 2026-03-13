//! Core types for the virtual list implementation.

/// A unique key for identifying items in the virtualizer.
pub(crate) type Key = usize;

/// A single virtualized item with computed position.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct VirtualItem {
    pub(crate) key: Key,
    pub(crate) index: usize,
    pub(crate) start: u32,
    pub(crate) end: u32,
    pub(crate) size: u32,
}

impl VirtualItem {
    pub(crate) fn new(key: Key, index: usize, start: u32, size: u32) -> Self {
        Self {
            key,
            index,
            start,
            end: start + size,
            size,
        }
    }
}

/// The visible range of items in the virtualizer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Range {
    pub(crate) start_index: usize,
    pub(crate) end_index: usize,
}

impl Range {
    pub(crate) fn new(start_index: usize, end_index: usize) -> Self {
        Self {
            start_index,
            end_index,
        }
    }
}
