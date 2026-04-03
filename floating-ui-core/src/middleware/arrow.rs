//! The arrow middleware — computes positioning for an arrow/caret element.

use euclid::SideOffsets2D;

use crate::middleware::{
    ArrowData, Middleware, MiddlewareDataEntry, MiddlewareReturn, MiddlewareState,
};
use crate::types::{Padding, Size};

/// Computes positioning data for an arrow element that points from the
/// floating element toward the reference element.
///
/// The arrow is positioned along the alignment axis (perpendicular to the
/// placement side), clamped to stay within the floating element's bounds.
pub struct Arrow {
    dimensions: Size,
    padding: Padding,
}

impl Arrow {
    /// Create with the given arrow element dimensions.
    pub fn new(dimensions: Size) -> Self {
        Self {
            dimensions,
            padding: SideOffsets2D::new_all_same(0.0),
        }
    }

    /// Set padding to keep the arrow from the edges of the floating element.
    pub fn padding(mut self, p: Padding) -> Self {
        self.padding = p;
        self
    }

    /// Set uniform padding on all sides.
    pub fn padding_all(mut self, p: f64) -> Self {
        self.padding = SideOffsets2D::new_all_same(p);
        self
    }
}

impl<E> Middleware<E> for Arrow {
    fn name(&self) -> &'static str {
        "arrow"
    }

    fn compute(&self, state: &MiddlewareState<E>) -> MiddlewareReturn {
        let placement = state.placement();
        let side = placement.side();
        let is_vertical = side.is_vertical();
        let rects = state.rects();

        let reference = rects.reference();
        let floating = rects.floating();
        let arrow_dimensions = self.dimensions;

        // Determine the alignment axis (perpendicular to the side)
        // For top/bottom: arrow moves along x
        // For left/right: arrow moves along y

        // Padding on the alignment axis edges
        let (padding_start, padding_end) = if is_vertical {
            (self.padding.left, self.padding.right)
        } else {
            (self.padding.top, self.padding.bottom)
        };

        // Arrow dimension on the alignment axis
        let arrow_len = if is_vertical {
            arrow_dimensions.width
        } else {
            arrow_dimensions.height
        };

        // Floating element dimension on the alignment axis
        let float_len = if is_vertical {
            floating.size.width
        } else {
            floating.size.height
        };

        // Reference element edges relative to floating
        let (ref_start, ref_end) = if is_vertical {
            let start = reference.origin.x - state.x();
            let end = start + reference.size.width;
            (start, end)
        } else {
            let start = reference.origin.y - state.y();
            let end = start + reference.size.height;
            (start, end)
        };

        // Where the arrow should ideally point (center of reference)
        let ref_center = (ref_start + ref_end) / 2.0;

        // Arrow positioning bounds within the floating element
        let min = padding_start;
        let max = float_len - arrow_len - padding_end;

        // Ideal center position for the arrow
        let ideal = ref_center - arrow_len / 2.0;

        // Clamp to bounds
        let clamped = ideal.clamp(min, max);

        // Center offset: how far the arrow had to shift from ideal center
        let center_offset = ideal - clamped;

        let (x, y) = if is_vertical {
            (Some(clamped), None)
        } else {
            (None, Some(clamped))
        };

        MiddlewareReturn::empty().with_data(MiddlewareDataEntry::Arrow(ArrowData::new(
            x,
            y,
            center_offset,
        )))
    }
}
