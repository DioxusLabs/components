//! Overflow detection — the foundation for collision-aware middleware.
//!
//! Computes how many pixels the floating element overflows each side of its
//! clipping boundary. Positive values indicate overflow; negative values
//! indicate remaining space.

use euclid::{Point2D, SideOffsets2D};

use crate::middleware::MiddlewareState;
use crate::types::{
    rect_to_clipping_rect, Boundary, ElementContext, LayoutRect, Overflow, Padding, PaddingInput,
    RootBoundary,
};

// ---------------------------------------------------------------------------
// DetectOverflowOptions
// ---------------------------------------------------------------------------

/// Configuration for overflow detection.
pub struct DetectOverflowOptions<E> {
    boundary: Boundary<E>,
    root_boundary: RootBoundary,
    element_context: ElementContext,
    alt_boundary: bool,
    padding: Padding,
}

impl<E> DetectOverflowOptions<E> {
    /// Create with defaults: `ClippingAncestors`, `Viewport`, `Floating`, no alt, no padding.
    pub fn new() -> Self {
        Self {
            boundary: Boundary::ClippingAncestors,
            root_boundary: RootBoundary::Viewport,
            element_context: ElementContext::Floating,
            alt_boundary: false,
            padding: SideOffsets2D::new_all_same(0.0),
        }
    }

    /// Set the clipping boundary.
    pub fn boundary(mut self, b: Boundary<E>) -> Self {
        self.boundary = b;
        self
    }

    /// Set the root boundary.
    pub fn root_boundary(mut self, rb: RootBoundary) -> Self {
        self.root_boundary = rb;
        self
    }

    /// Set which element to check overflow for.
    pub fn element_context(mut self, ec: ElementContext) -> Self {
        self.element_context = ec;
        self
    }

    /// Use the alternate element's boundary (e.g., check the floating element
    /// against the reference element's clipping boundary).
    pub fn alt_boundary(mut self, ab: bool) -> Self {
        self.alt_boundary = ab;
        self
    }

    /// Set uniform padding.
    pub fn padding_all(mut self, p: f64) -> Self {
        self.padding = SideOffsets2D::new_all_same(p);
        self
    }

    /// Set per-side padding.
    pub fn padding(mut self, p: Padding) -> Self {
        self.padding = p;
        self
    }

    /// Set padding from a `PaddingInput`.
    pub fn padding_input(mut self, p: PaddingInput) -> Self {
        self.padding = p.to_padding();
        self
    }

    /// Access the current boundary.
    pub fn get_boundary(&self) -> &Boundary<E> {
        &self.boundary
    }

    /// Access the current root boundary.
    pub fn get_root_boundary(&self) -> RootBoundary {
        self.root_boundary
    }

    /// Access the current element context.
    pub fn get_element_context(&self) -> ElementContext {
        self.element_context
    }

    /// Access the alt-boundary flag.
    pub fn get_alt_boundary(&self) -> bool {
        self.alt_boundary
    }

    /// Access the current padding.
    pub fn get_padding(&self) -> &Padding {
        &self.padding
    }
}

impl<E> Default for DetectOverflowOptions<E> {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// detect_overflow
// ---------------------------------------------------------------------------

/// Compute how many pixels the element overflows each side of its clipping boundary.
///
/// Returns a [`SideOffsets2D`] where:
/// - **Positive** values indicate pixels of overflow beyond the boundary.
/// - **Negative** values indicate pixels of remaining space before the boundary.
pub fn detect_overflow<E>(
    state: &MiddlewareState<'_, E>,
    options: &DetectOverflowOptions<E>,
) -> Overflow {
    let platform = state.platform();

    // Determine which element to measure and which to clip against
    let (check_element, clip_element) = if options.alt_boundary {
        match options.element_context {
            ElementContext::Floating => (state.elements().floating(), state.elements().reference()),
            ElementContext::Reference => {
                (state.elements().reference(), state.elements().floating())
            }
        }
    } else {
        let elem = match options.element_context {
            ElementContext::Floating => state.elements().floating(),
            ElementContext::Reference => state.elements().reference(),
        };
        (elem, elem)
    };

    // Get the clipping rect from the platform
    let clipping_rect = platform.get_clipping_rect(
        clip_element,
        &options.boundary,
        options.root_boundary,
        state.strategy(),
    );

    // Get the element rect to check
    let element_rect = match options.element_context {
        ElementContext::Floating => {
            // Build the floating rect at the current coordinates
            LayoutRect::new(
                Point2D::new(state.x(), state.y()),
                state.rects().floating().size,
            )
        }
        ElementContext::Reference => state.rects().reference(),
    };

    // Convert to viewport-relative coordinates if we have an offset parent
    let element_rect = if let Some(offset_parent) = platform.get_offset_parent(check_element) {
        platform.convert_offset_parent_relative_rect_to_viewport_relative_rect(
            element_rect,
            &offset_parent,
            state.strategy(),
        )
    } else {
        element_rect
    };

    let element_clip = rect_to_clipping_rect(element_rect);

    // Compute overflow per side
    // Positive = overflow, negative = remaining space
    let top = clipping_rect.min.y - element_clip.min.y + options.padding.top;
    let bottom = element_clip.max.y - clipping_rect.max.y + options.padding.bottom;
    let left = clipping_rect.min.x - element_clip.min.x + options.padding.left;
    let right = element_clip.max.x - clipping_rect.max.x + options.padding.right;

    SideOffsets2D::new(top, right, bottom, left)
}
