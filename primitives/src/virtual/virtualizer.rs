//! Core Virtualizer implementation.

use std::collections::HashMap;

use super::types::{Key, Range, Rect, ScrollDirection, VirtualItem};
use super::utils::{default_range_extractor, find_nearest_binary_search};

/// Core virtualizer that manages item positions and scroll state.
///
/// This struct is framework-agnostic and contains pure algorithms for:
/// - Computing item positions from sizes
/// - Calculating the visible range
/// - Handling scroll position corrections when items resize
#[derive(Debug)]
pub struct Virtualizer<F, K>
where
    F: Fn(usize) -> u32,
    K: Fn(usize) -> Key,
{
    // Configuration
    count: usize,
    estimate_size: F,
    get_item_key: K,
    overscan: usize,
    padding_start: u32,
    padding_end: u32,
    gap: u32,
    lanes: usize,
    horizontal: bool,

    // Measurements cache
    measurements_cache: Vec<VirtualItem>,
    item_size_cache: HashMap<Key, u32>,
    pending_measured_indexes: Vec<usize>,
    measurements_dirty: bool,

    // Adaptive estimation: track sum and count of measured items
    measured_sizes_sum: u64,
    measured_sizes_count: usize,
    use_adaptive_estimation: bool,

    // Stable total size (frozen during scrolling to prevent scrollbar drift)
    stable_total_size: Option<u32>,

    // Accumulated scroll adjustment during scrolling (for items above viewport)
    deferred_adjustments: i32,

    // Scroll state
    scroll_offset: u32,
    scroll_rect: Rect,
    scroll_direction: Option<ScrollDirection>,
    scroll_adjustments: i32,
    is_scrolling: bool,

    // Range cache
    range: Option<Range>,
    range_dirty: bool,
}

/// Configuration options for creating a Virtualizer.
#[derive(Debug, Clone)]
pub struct VirtualizerOptions<F, K>
where
    F: Fn(usize) -> u32,
    K: Fn(usize) -> Key,
{
    /// Total number of items.
    pub count: usize,
    /// Function to estimate the size of an item before measurement.
    pub estimate_size: F,
    /// Function to get a unique key for an item.
    pub get_item_key: K,
    /// Number of items to render outside the visible range (default: 1).
    pub overscan: usize,
    /// Padding at the start of the list in pixels.
    pub padding_start: u32,
    /// Padding at the end of the list in pixels.
    pub padding_end: u32,
    /// Gap between items in pixels.
    pub gap: u32,
    /// Number of lanes (columns) for masonry/grid layouts (default: 1).
    pub lanes: usize,
    /// Whether to virtualize horizontally instead of vertically.
    pub horizontal: bool,
    /// Whether to use adaptive estimation (average of measured sizes) instead of estimate_size.
    pub use_adaptive_estimation: bool,
}

impl<F, K> VirtualizerOptions<F, K>
where
    F: Fn(usize) -> u32,
    K: Fn(usize) -> Key,
{
    /// Creates a new VirtualizerOptions with required fields and defaults.
    pub fn new(count: usize, estimate_size: F, get_item_key: K) -> Self {
        Self {
            count,
            estimate_size,
            get_item_key,
            overscan: 1,
            padding_start: 0,
            padding_end: 0,
            gap: 0,
            lanes: 1,
            horizontal: false,
            use_adaptive_estimation: false,
        }
    }

    /// Sets the overscan value.
    pub fn overscan(mut self, overscan: usize) -> Self {
        self.overscan = overscan;
        self
    }

    /// Sets whether to use adaptive estimation (average of measured sizes).
    pub fn use_adaptive_estimation(mut self, use_adaptive: bool) -> Self {
        self.use_adaptive_estimation = use_adaptive;
        self
    }
}

impl<F, K> Virtualizer<F, K>
where
    F: Fn(usize) -> u32,
    K: Fn(usize) -> Key,
{
    /// Creates a new Virtualizer with the given options.
    pub fn new(options: VirtualizerOptions<F, K>) -> Self {
        Self {
            count: options.count,
            estimate_size: options.estimate_size,
            get_item_key: options.get_item_key,
            overscan: options.overscan,
            padding_start: options.padding_start,
            padding_end: options.padding_end,
            gap: options.gap,
            lanes: options.lanes.max(1),
            horizontal: options.horizontal,

            measurements_cache: Vec::new(),
            item_size_cache: HashMap::new(),
            pending_measured_indexes: Vec::new(),
            measurements_dirty: true,

            measured_sizes_sum: 0,
            measured_sizes_count: 0,
            use_adaptive_estimation: options.use_adaptive_estimation,

            stable_total_size: None,
            deferred_adjustments: 0,

            scroll_offset: 0,
            scroll_rect: Rect::default(),
            scroll_direction: None,
            scroll_adjustments: 0,
            is_scrolling: false,

            range: None,
            range_dirty: true,
        }
    }

    /// Updates the item count. Call this when the data source changes.
    pub fn set_count(&mut self, count: usize) {
        if self.count != count {
            self.count = count;
            self.measurements_dirty = true;
            self.range_dirty = true;
        }
    }

    /// Updates the scroll offset. Call this from scroll event handlers.
    ///
    /// Returns an optional scroll correction to apply when scrolling stops,
    /// to compensate for height changes that occurred during scrolling.
    pub fn set_scroll_offset(&mut self, offset: u32, is_scrolling: bool) -> Option<i32> {
        let prev_offset = self.scroll_offset;
        let was_scrolling = self.is_scrolling;
        let mut correction = None;

        // Detect scroll direction
        if is_scrolling && offset != prev_offset {
            self.scroll_direction = Some(if offset > prev_offset {
                ScrollDirection::Forward
            } else {
                ScrollDirection::Backward
            });
        } else if !is_scrolling {
            self.scroll_direction = None;
        }

        // Reset adjustments when user starts a new scroll
        if is_scrolling && !was_scrolling {
            self.scroll_adjustments = 0;
            self.deferred_adjustments = 0;
            // Freeze total size when scrolling starts to prevent scrollbar drift
            self.stable_total_size = Some(self.calculate_total_size());
        }

        // When scrolling stops, apply accumulated deferred adjustments
        if !is_scrolling && was_scrolling {
            self.stable_total_size = None;

            // Return accumulated adjustments from items measured above viewport during scrolling
            if self.deferred_adjustments != 0 {
                correction = Some(self.deferred_adjustments);
                self.deferred_adjustments = 0;
            }
        }

        self.scroll_offset = offset;
        self.is_scrolling = is_scrolling;
        self.range_dirty = true;

        correction
    }

    /// Updates only the scroll offset for range calculation, without affecting
    /// the is_scrolling state or stable_total_size. Use this in render functions
    /// where you need to sync the offset but don't want to interfere with scroll
    /// state transitions managed by event handlers.
    pub fn sync_scroll_offset(&mut self, offset: u32) {
        if self.scroll_offset != offset {
            // If scroll position changed and we haven't frozen yet, freeze now.
            if self.stable_total_size.is_none() {
                self.stable_total_size = Some(self.calculate_total_size());
            }
            self.scroll_offset = offset;
            self.range_dirty = true;
        }
    }

    /// Returns the scroll offset with pending adjustments applied.
    fn adjusted_scroll_offset(&self) -> u32 {
        (self.scroll_offset as i32 + self.scroll_adjustments).max(0) as u32
    }

    /// Updates the viewport size. Call this from resize event handlers.
    pub fn set_viewport_size(&mut self, rect: Rect) {
        if self.scroll_rect != rect {
            self.scroll_rect = rect;
            self.range_dirty = true;
        }
    }

    /// Returns the viewport size.
    fn viewport_size(&self) -> u32 {
        self.scroll_rect.size(self.horizontal)
    }

    /// Resizes an item and returns the scroll adjustment if needed.
    ///
    /// This is the **key method for preventing scroll jumping**. When an item
    /// above the viewport changes size, we need to adjust the scroll position
    /// by the size delta to keep the visible content stable.
    pub fn resize_item(&mut self, index: usize, new_size: u32) -> Option<i32> {
        // Ensure measurements are up to date
        self.ensure_measurements();

        let item = self.measurements_cache.get(index)?;
        let key = item.key;

        // If already measured, only update if significantly different (>2px)
        if let Some(&cached_size) = self.item_size_cache.get(&key) {
            let remeasure_delta = (new_size as i32 - cached_size as i32).abs();
            if remeasure_delta <= 2 {
                return None;
            }
        }

        let old_size = self.item_size_cache.get(&key).copied().unwrap_or(item.size);
        let delta = new_size as i32 - old_size as i32;

        // For tiny deltas (sub-pixel rounding), still cache but don't adjust scroll
        let significant_delta = delta.abs() > 1;

        if delta == 0 {
            return None;
        }

        // Only adjust scroll for items ABOVE the viewport.
        let adjusted_scroll = self.adjusted_scroll_offset();
        let is_above_viewport = item.start < adjusted_scroll;

        let should_adjust_now =
            significant_delta && !self.is_scrolling && is_above_viewport;

        // Update the size cache and adaptive estimation tracking
        let was_measured = self.item_size_cache.contains_key(&key);
        if was_measured {
            self.measured_sizes_sum = self.measured_sizes_sum.saturating_sub(old_size as u64);
            self.measured_sizes_sum = self.measured_sizes_sum.saturating_add(new_size as u64);
        } else {
            self.measured_sizes_sum = self.measured_sizes_sum.saturating_add(new_size as u64);
            self.measured_sizes_count += 1;
        }

        self.item_size_cache.insert(key, new_size);
        self.pending_measured_indexes.push(index);
        self.measurements_dirty = true;
        self.range_dirty = true;

        if should_adjust_now {
            self.scroll_adjustments += delta;
            return Some(delta);
        } else if significant_delta && self.is_scrolling && is_above_viewport {
            // Defer scroll adjustment until scrolling stops
            self.deferred_adjustments += delta;
        }

        None
    }

    /// Returns the estimated size for an item.
    fn get_estimate(&self, index: usize) -> u32 {
        if self.use_adaptive_estimation && self.measured_sizes_count > 0 {
            (self.measured_sizes_sum / self.measured_sizes_count as u64) as u32
        } else {
            (self.estimate_size)(index)
        }
    }

    /// Ensures measurements are up to date.
    fn ensure_measurements(&mut self) {
        if self.measurements_dirty {
            self.recalculate_measurements();
            self.measurements_dirty = false;
        }
    }

    /// Recalculates all item measurements.
    fn recalculate_measurements(&mut self) {
        if self.count == 0 {
            self.measurements_cache.clear();
            return;
        }

        // Determine the starting index for recalculation
        let min_index = if self.pending_measured_indexes.is_empty() {
            0
        } else {
            *self.pending_measured_indexes.iter().min().unwrap_or(&0)
        };
        self.pending_measured_indexes.clear();

        // Reuse existing measurements up to min_index
        let mut measurements: Vec<VirtualItem> =
            if min_index > 0 && min_index < self.measurements_cache.len() {
                self.measurements_cache[..min_index].to_vec()
            } else {
                Vec::with_capacity(self.count)
            };

        if self.lanes == 1 {
            // Single lane: simple sequential layout
            for i in measurements.len()..self.count {
                let key = (self.get_item_key)(i);
                let size = self
                    .item_size_cache
                    .get(&key)
                    .copied()
                    .unwrap_or_else(|| self.get_estimate(i));

                let start = measurements
                    .last()
                    .map(|m| m.end + self.gap)
                    .unwrap_or(self.padding_start);

                measurements.push(VirtualItem::new(key, i, start, size, 0));
            }
        } else {
            // Multi-lane: masonry layout
            self.recalculate_multi_lane(&mut measurements);
        }

        self.measurements_cache = measurements;
    }

    /// Recalculates measurements for multi-lane (masonry) layout.
    fn recalculate_multi_lane(&mut self, measurements: &mut Vec<VirtualItem>) {
        let mut lane_ends: Vec<u32> = vec![self.padding_start; self.lanes];

        for item in measurements.iter() {
            if item.lane < self.lanes {
                lane_ends[item.lane] = item.end;
            }
        }

        for i in measurements.len()..self.count {
            let key = (self.get_item_key)(i);
            let size = self
                .item_size_cache
                .get(&key)
                .copied()
                .unwrap_or_else(|| self.get_estimate(i));

            let (lane, &lane_end) = lane_ends
                .iter()
                .enumerate()
                .min_by_key(|(_, &end)| end)
                .unwrap();

            let start = if lane_end > self.padding_start {
                lane_end + self.gap
            } else {
                lane_end
            };

            measurements.push(VirtualItem::new(key, i, start, size, lane));
            lane_ends[lane] = start + size;
        }
    }

    /// Calculates the visible range based on current scroll position.
    fn calculate_range(&mut self) -> Option<Range> {
        self.ensure_measurements();

        if self.measurements_cache.is_empty() || self.viewport_size() == 0 {
            return None;
        }

        let scroll_offset = self.scroll_offset;
        let viewport_size = self.viewport_size();
        let measurements = &self.measurements_cache;

        // Handle case when item count <= lanes
        if measurements.len() <= self.lanes {
            return Some(Range::new(0, measurements.len() - 1));
        }

        // Find start index using binary search
        let mut start_index = find_nearest_binary_search(measurements, scroll_offset);
        let mut end_index = start_index;
        let last_index = measurements.len() - 1;

        if self.lanes == 1 {
            while end_index < last_index
                && measurements[end_index].end < scroll_offset + viewport_size
            {
                end_index += 1;
            }
        } else {
            let mut end_per_lane = vec![0u32; self.lanes];
            while end_index < last_index
                && end_per_lane
                    .iter()
                    .any(|&pos| pos < scroll_offset + viewport_size)
            {
                let item = &measurements[end_index];
                end_per_lane[item.lane] = item.end;
                end_index += 1;
            }

            let mut start_per_lane = vec![scroll_offset + viewport_size; self.lanes];
            while start_index > 0 && start_per_lane.iter().any(|&pos| pos >= scroll_offset) {
                let item = &measurements[start_index];
                start_per_lane[item.lane] = item.start;
                start_index = start_index.saturating_sub(1);
            }
        }

        Some(Range::new(start_index, end_index))
    }

    /// Returns the visible range, calculating if needed.
    fn get_range(&mut self) -> Option<Range> {
        if self.range_dirty {
            self.range = self.calculate_range();
            self.range_dirty = false;
        }
        self.range
    }

    /// Returns the indices of items to render (with overscan applied).
    fn get_virtual_indexes(&mut self) -> Vec<usize> {
        let range = match self.get_range() {
            Some(r) => r,
            None => return Vec::new(),
        };

        default_range_extractor(
            range.start_index,
            range.end_index,
            self.overscan,
            self.count,
        )
    }

    /// Returns the virtual items to render.
    pub fn get_virtual_items(&mut self) -> Vec<VirtualItem> {
        let indexes = self.get_virtual_indexes();
        self.ensure_measurements();

        indexes
            .into_iter()
            .filter_map(|i| self.measurements_cache.get(i).cloned())
            .collect()
    }

    /// Returns the total scrollable size.
    /// During active scrolling, returns a frozen value to prevent scrollbar drift.
    pub fn get_total_size(&mut self) -> u32 {
        if let Some(stable) = self.stable_total_size {
            return stable;
        }
        self.calculate_total_size()
    }

    /// Calculates the actual total size from measurements.
    fn calculate_total_size(&mut self) -> u32 {
        self.ensure_measurements();

        if self.measurements_cache.is_empty() {
            return self.padding_start + self.padding_end;
        }

        let end = if self.lanes == 1 {
            self.measurements_cache.last().map(|m| m.end).unwrap_or(0)
        } else {
            let mut max_end_per_lane = vec![0u32; self.lanes];
            for item in self.measurements_cache.iter().rev() {
                if max_end_per_lane[item.lane] == 0 {
                    max_end_per_lane[item.lane] = item.end;
                }
                if max_end_per_lane.iter().all(|&e| e > 0) {
                    break;
                }
            }
            max_end_per_lane.into_iter().max().unwrap_or(0)
        };

        (end + self.padding_end).max(self.padding_start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_virtualizer() -> Virtualizer<impl Fn(usize) -> u32, impl Fn(usize) -> Key> {
        let options = VirtualizerOptions::new(100, |_| 50, |i| i).overscan(2);
        let mut v = Virtualizer::new(options);
        v.set_viewport_size(Rect::new(800, 600));
        v
    }

    #[test]
    fn test_total_size() {
        let mut v = create_test_virtualizer();
        // 100 items * 50px each = 5000px
        assert_eq!(v.get_total_size(), 5000);
    }

    #[test]
    fn test_virtual_items_with_overscan() {
        let mut v = create_test_virtualizer();
        v.set_scroll_offset(500, false); // Scroll to item 10

        let items = v.get_virtual_items();

        // Should include overscan items before and after
        assert!(!items.is_empty());
        assert!(items.first().unwrap().index <= 10);
    }

    #[test]
    fn test_resize_item_below_viewport() {
        let mut v = create_test_virtualizer();
        v.set_scroll_offset(0, false);

        // Resize an item below the viewport - should not cause adjustment
        let adjustment = v.resize_item(50, 100);
        assert!(adjustment.is_none());
    }

    #[test]
    fn test_resize_item_above_viewport() {
        let mut v = create_test_virtualizer();
        v.set_scroll_offset(1000, false); // Scroll to item 20

        // Resize an item above the viewport
        let adjustment = v.resize_item(5, 100); // Item 5 is above viewport
        assert!(adjustment.is_some());
        assert_eq!(adjustment.unwrap(), 50); // Delta: 100 - 50 = 50
    }

    #[test]
    fn test_deferred_adjustments_during_scrolling() {
        let mut v = create_test_virtualizer();

        // Start scrolling to item 20 (offset 1000)
        v.set_scroll_offset(1000, true);

        // Resize item 5 (above viewport) while scrolling
        let adjustment = v.resize_item(5, 100); // 100 - 50 = +50
        assert!(adjustment.is_none(), "Should not adjust during scrolling");

        // Resize another item above viewport
        let adjustment = v.resize_item(3, 80); // 80 - 50 = +30
        assert!(adjustment.is_none(), "Should not adjust during scrolling");

        // When scrolling stops, should return accumulated deferred adjustments
        let correction = v.set_scroll_offset(1000, false);
        assert_eq!(
            correction,
            Some(80),
            "Should return accumulated delta: 50 + 30 = 80"
        );
    }

    #[test]
    fn test_no_deferred_adjustments_for_items_below_viewport() {
        let mut v = create_test_virtualizer();

        // Start scrolling to item 20 (offset 1000)
        v.set_scroll_offset(1000, true);

        // Resize item 50 (below viewport) while scrolling
        let adjustment = v.resize_item(50, 100);
        assert!(adjustment.is_none());

        // When scrolling stops, should return no correction since item was below viewport
        let correction = v.set_scroll_offset(1000, false);
        assert!(
            correction.is_none(),
            "No deferred adjustment for items below viewport"
        );
    }
}
