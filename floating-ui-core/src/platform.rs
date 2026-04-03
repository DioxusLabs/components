//! The platform abstraction trait.
//!
//! All DOM/environment measurement operations are delegated to the [`Platform`]
//! trait, making the core positioning engine platform-agnostic.

use crate::types::{
    Boundary, ClientRect, ClippingRect, ElementRects, LayoutRect, RootBoundary, Size, Strategy,
};

/// Abstraction over the rendering environment (DOM, React Native, Canvas, etc.).
///
/// The core positioning engine calls these methods to measure elements and
/// determine clipping boundaries. Implementations are generic over the element
/// type `E` — for example, a DOM platform uses `web_sys::Element`.
///
/// Three methods are required; all others have sensible defaults.
pub trait Platform<E> {
    /// Measure both the reference and floating elements, returning their
    /// bounding rectangles relative to the floating element's offset parent.
    fn get_element_rects(&self, reference: &E, floating: &E, strategy: Strategy) -> ElementRects;

    /// Compute the visible clipping boundary for the given element.
    ///
    /// This is typically the intersection of all overflow-clipping ancestors
    /// with the root boundary (viewport or document).
    fn get_clipping_rect(
        &self,
        element: &E,
        boundary: &Boundary<E>,
        root_boundary: RootBoundary,
        strategy: Strategy,
    ) -> ClippingRect;

    /// Return the dimensions (width, height) of the given element.
    fn get_dimensions(&self, element: &E) -> Size;

    /// Convert a rect that is relative to the offset parent into one that is
    /// relative to the viewport.
    fn convert_offset_parent_relative_rect_to_viewport_relative_rect(
        &self,
        rect: LayoutRect,
        _offset_parent: &E,
        _strategy: Strategy,
    ) -> LayoutRect {
        rect
    }

    /// Return the offset parent of the given element.
    fn get_offset_parent(&self, _element: &E) -> Option<E> {
        None
    }

    /// Check whether the given value is a real element (as opposed to a window or virtual element).
    fn is_element(&self, _value: &E) -> bool {
        true
    }

    /// Return the document element (e.g., `<html>`) for the given element.
    fn get_document_element(&self, _element: &E) -> Option<E> {
        None
    }

    /// Return the individual client rects for multi-line inline elements.
    fn get_client_rects(&self, _element: &E) -> Vec<ClientRect> {
        vec![]
    }

    /// Whether the element's writing direction is right-to-left.
    fn is_rtl(&self, _element: &E) -> bool {
        false
    }

    /// Return the CSS `transform` scale of the element as `(scale_x, scale_y)`.
    fn get_scale(&self, _element: &E) -> (f64, f64) {
        (1.0, 1.0)
    }
}
