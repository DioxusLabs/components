//! Utility functions for the virtual list implementation.
//!
//! These utilities are framework-agnostic and mirror TanStack Virtual's utils.

use super::types::VirtualItem;

/// Approximate equality for scroll positions.
///
/// Returns true if the two values are within 1 pixel of each other.
/// This is used to determine if scroll has settled at the target position.
#[inline]
pub fn approx_equal(a: u32, b: u32) -> bool {
    (a as i32 - b as i32).abs() < 2
}

/// Binary search to find the nearest item at or before the given offset.
///
/// Returns the index of the item whose `start` position is closest to
/// (but not exceeding) the given offset. This is used for calculating
/// the visible range.
///
/// # Arguments
/// * `measurements` - The sorted list of virtual items
/// * `offset` - The scroll offset to search for
///
/// # Returns
/// The index of the nearest item, or 0 if measurements is empty.
pub fn find_nearest_binary_search(measurements: &[VirtualItem], offset: u32) -> usize {
    if measurements.is_empty() {
        return 0;
    }

    let mut low = 0usize;
    let mut high = measurements.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;
        let current = measurements[mid].start;

        if current < offset {
            low = mid + 1;
        } else if current > offset {
            if mid == 0 {
                break;
            }
            high = mid - 1;
        } else {
            return mid;
        }
    }

    low.saturating_sub(1)
}

/// Extract indices from a range with overscan applied.
///
/// This is the default range extractor that adds overscan items
/// before and after the visible range.
///
/// # Arguments
/// * `start_index` - First visible item index
/// * `end_index` - Last visible item index
/// * `overscan` - Number of items to render outside visible range
/// * `count` - Total number of items
///
/// # Returns
/// A vector of indices to render.
pub fn default_range_extractor(
    start_index: usize,
    end_index: usize,
    overscan: usize,
    count: usize,
) -> Vec<usize> {
    if count == 0 {
        return Vec::new();
    }

    let start = start_index.saturating_sub(overscan);
    let end = (end_index + overscan).min(count - 1);

    (start..=end).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_approx_equal() {
        assert!(approx_equal(100, 100));
        assert!(approx_equal(100, 101));
        assert!(approx_equal(101, 100));
        assert!(!approx_equal(100, 102));
        assert!(!approx_equal(102, 100));
    }

    #[test]
    fn test_find_nearest_binary_search_empty() {
        let measurements: Vec<VirtualItem> = vec![];
        assert_eq!(find_nearest_binary_search(&measurements, 100), 0);
    }

    #[test]
    fn test_find_nearest_binary_search_single() {
        let measurements = vec![VirtualItem::new(0, 0, 0, 100, 0)];
        assert_eq!(find_nearest_binary_search(&measurements, 0), 0);
        assert_eq!(find_nearest_binary_search(&measurements, 50), 0);
        assert_eq!(find_nearest_binary_search(&measurements, 100), 0);
    }

    #[test]
    fn test_find_nearest_binary_search_multiple() {
        // Items at offsets: 0, 100, 200, 300, 400
        let measurements: Vec<VirtualItem> = (0..5)
            .map(|i| VirtualItem::new(i, i, i as u32 * 100, 100, 0))
            .collect();

        assert_eq!(find_nearest_binary_search(&measurements, 0), 0);
        assert_eq!(find_nearest_binary_search(&measurements, 50), 0);
        assert_eq!(find_nearest_binary_search(&measurements, 100), 1);
        assert_eq!(find_nearest_binary_search(&measurements, 150), 1);
        assert_eq!(find_nearest_binary_search(&measurements, 250), 2);
        assert_eq!(find_nearest_binary_search(&measurements, 400), 4);
        assert_eq!(find_nearest_binary_search(&measurements, 500), 4);
    }

    #[test]
    fn test_default_range_extractor() {
        // Empty
        assert_eq!(default_range_extractor(0, 0, 1, 0), Vec::<usize>::new());

        // No overscan needed
        assert_eq!(
            default_range_extractor(2, 5, 2, 10),
            vec![0, 1, 2, 3, 4, 5, 6, 7]
        );

        // Overscan at start boundary
        assert_eq!(default_range_extractor(0, 3, 2, 10), vec![0, 1, 2, 3, 4, 5]);

        // Overscan at end boundary
        assert_eq!(default_range_extractor(7, 9, 2, 10), vec![5, 6, 7, 8, 9]);
    }
}
