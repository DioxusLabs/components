//! A pre-measured platform for environments where DOM measurements
//! are obtained asynchronously (e.g., through eval-based JS interop).
//!
//! This allows frameworks like Dioxus to measure elements once via JS,
//! then run the pure Rust positioning algorithm synchronously.

use euclid::Point2D;

use crate::platform::Platform;
use crate::types::*;

/// Phantom element type for pre-measured positioning.
///
/// Since measurements are taken ahead of time, no real element
/// references are needed during [`compute_position`](crate::compute_position::compute_position).
pub struct PreMeasured;

/// A [`Platform`] implementation that uses pre-computed measurements
/// instead of live DOM queries.
///
/// Create one by passing in the measured rectangles from JS, then
/// use it with [`compute_position`](crate::compute_position::compute_position).
pub struct PreMeasuredPlatform {
    reference_rect: LayoutRect,
    floating_size: Size,
    clipping_rect: ClippingRect,
    is_rtl: bool,
}

impl PreMeasuredPlatform {
    /// Create a new pre-measured platform.
    ///
    /// - `reference_rect`: The reference element's bounding client rect
    ///   (from `getBoundingClientRect()`).
    /// - `floating_size`: The floating element's width and height.
    /// - `clipping_rect`: The viewport or clipping boundary as a `Box2D`.
    /// - `is_rtl`: Whether the layout direction is right-to-left.
    pub fn new(
        reference_rect: LayoutRect,
        floating_size: Size,
        clipping_rect: ClippingRect,
        is_rtl: bool,
    ) -> Self {
        Self {
            reference_rect,
            floating_size,
            clipping_rect,
            is_rtl,
        }
    }
}

impl Platform<PreMeasured> for PreMeasuredPlatform {
    fn get_element_rects(
        &self,
        _reference: &PreMeasured,
        _floating: &PreMeasured,
        _strategy: Strategy,
    ) -> ElementRects {
        ElementRects::new(
            self.reference_rect,
            LayoutRect::new(Point2D::new(0.0, 0.0), self.floating_size),
        )
    }

    fn get_clipping_rect(
        &self,
        _element: &PreMeasured,
        _boundary: &Boundary<PreMeasured>,
        _root_boundary: RootBoundary,
        _strategy: Strategy,
    ) -> ClippingRect {
        self.clipping_rect
    }

    fn get_dimensions(&self, _element: &PreMeasured) -> Size {
        self.floating_size
    }

    fn is_rtl(&self, _element: &PreMeasured) -> bool {
        self.is_rtl
    }
}
