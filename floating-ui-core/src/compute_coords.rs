//! Compute initial coordinates from a placement and element rects.

use euclid::Point2D;

use crate::types::{Alignment, ElementRects, Placement, Point, Side};

/// Compute the initial `(x, y)` coordinates for the floating element based
/// on the placement and element rectangles.
///
/// This positions the floating element adjacent to the reference element on the
/// specified side, centered or aligned along the cross axis.
///
/// When `rtl` is true and the placement is vertical (top/bottom), the start/end
/// alignment direction is inverted.
pub fn compute_coords_from_placement(
    rects: &ElementRects,
    placement: Placement,
    rtl: bool,
) -> Point {
    let reference = rects.reference();
    let floating = rects.floating();

    let side = placement.side();
    let alignment = placement.alignment();
    let is_vertical = side.is_vertical();

    // Common center positions
    let common_x = reference.origin.x + reference.size.width / 2.0 - floating.size.width / 2.0;
    let common_y = reference.origin.y + reference.size.height / 2.0 - floating.size.height / 2.0;

    // Base coordinates from side
    let (mut x, mut y) = match side {
        Side::Top => (common_x, reference.origin.y - floating.size.height),
        Side::Bottom => (common_x, reference.max_y()),
        Side::Left => (reference.origin.x - floating.size.width, common_y),
        Side::Right => (reference.max_x(), common_y),
    };

    // Alignment adjustment along the cross axis
    if let Some(align) = alignment {
        let (ref_len, float_len) = if is_vertical {
            (reference.size.width, floating.size.width)
        } else {
            (reference.size.height, floating.size.height)
        };

        let half_diff = (ref_len - float_len) / 2.0;

        // RTL inverts alignment for vertical placements (top/bottom)
        let rtl_multiplier = if rtl && is_vertical { -1.0 } else { 1.0 };

        let alignment_offset = match align {
            Alignment::Start => half_diff * rtl_multiplier,
            Alignment::End => -half_diff * rtl_multiplier,
        };

        if is_vertical {
            // Vertical placement → align along x
            x -= alignment_offset;
        } else {
            // Horizontal placement → align along y
            y -= alignment_offset;
        }
    }

    Point2D::new(x, y)
}

#[cfg(test)]
mod tests {
    use euclid::{Point2D, Size2D};

    use super::*;
    use crate::types::{ElementRects, LayoutRect};

    fn make_rects(
        ref_x: f64,
        ref_y: f64,
        ref_w: f64,
        ref_h: f64,
        float_w: f64,
        float_h: f64,
    ) -> ElementRects {
        ElementRects::new(
            LayoutRect::new(Point2D::new(ref_x, ref_y), Size2D::new(ref_w, ref_h)),
            LayoutRect::new(Point2D::new(0.0, 0.0), Size2D::new(float_w, float_h)),
        )
    }

    #[test]
    fn test_bottom_centered() {
        let rects = make_rects(100.0, 100.0, 200.0, 50.0, 100.0, 30.0);
        let p = compute_coords_from_placement(&rects, Placement::Bottom, false);
        // x: 100 + 200/2 - 100/2 = 150
        // y: 100 + 50 = 150
        assert_eq!(p, Point2D::new(150.0, 150.0));
    }

    #[test]
    fn test_top_centered() {
        let rects = make_rects(100.0, 100.0, 200.0, 50.0, 100.0, 30.0);
        let p = compute_coords_from_placement(&rects, Placement::Top, false);
        // x: 150
        // y: 100 - 30 = 70
        assert_eq!(p, Point2D::new(150.0, 70.0));
    }

    #[test]
    fn test_right_centered() {
        let rects = make_rects(100.0, 100.0, 200.0, 50.0, 100.0, 30.0);
        let p = compute_coords_from_placement(&rects, Placement::Right, false);
        // x: 100 + 200 = 300
        // y: 100 + 50/2 - 30/2 = 110
        assert_eq!(p, Point2D::new(300.0, 110.0));
    }

    #[test]
    fn test_left_centered() {
        let rects = make_rects(100.0, 100.0, 200.0, 50.0, 100.0, 30.0);
        let p = compute_coords_from_placement(&rects, Placement::Left, false);
        // x: 100 - 100 = 0
        // y: 110
        assert_eq!(p, Point2D::new(0.0, 110.0));
    }

    #[test]
    fn test_bottom_start() {
        let rects = make_rects(100.0, 100.0, 200.0, 50.0, 100.0, 30.0);
        let p = compute_coords_from_placement(&rects, Placement::BottomStart, false);
        // half_diff = (200 - 100) / 2 = 50
        // x: 150 - 50 = 100 (aligned to start = left edge of reference)
        // y: 150
        assert_eq!(p, Point2D::new(100.0, 150.0));
    }

    #[test]
    fn test_bottom_end() {
        let rects = make_rects(100.0, 100.0, 200.0, 50.0, 100.0, 30.0);
        let p = compute_coords_from_placement(&rects, Placement::BottomEnd, false);
        // half_diff = 50
        // x: 150 - (-50) = 200 (aligned to end = right edge of reference)
        // y: 150
        assert_eq!(p, Point2D::new(200.0, 150.0));
    }

    #[test]
    fn test_bottom_start_rtl() {
        let rects = make_rects(100.0, 100.0, 200.0, 50.0, 100.0, 30.0);
        let p = compute_coords_from_placement(&rects, Placement::BottomStart, true);
        // RTL inverts: start becomes end
        // half_diff = 50, rtl_multiplier = -1
        // alignment_offset = 50 * -1 = -50
        // x: 150 - (-50) = 200
        assert_eq!(p, Point2D::new(200.0, 150.0));
    }
}
