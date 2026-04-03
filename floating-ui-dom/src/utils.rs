//! DOM traversal and measurement utilities.
//!
//! These helpers are used by [`DomPlatform`](crate::platform::DomPlatform) to
//! traverse the DOM tree, find overflow ancestors, compute clipping boundaries,
//! and measure elements.

use euclid::{Box2D, Point2D, Rect, Size2D};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, Window};

use floating_ui_core::types::{ClippingRect, LayoutRect};

/// Get the global `Window` object.
pub(crate) fn get_window() -> Window {
    web_sys::window().expect("no global window")
}

/// Get the `Document` from the window.
pub(crate) fn get_document() -> web_sys::Document {
    get_window().document().expect("no document")
}

/// Get the `<html>` document element.
pub(crate) fn get_document_element() -> Element {
    get_document()
        .document_element()
        .expect("no document element")
}

/// Check whether the element has overflow clipping (hidden, scroll, auto, clip).
pub(crate) fn is_overflow_element(element: &Element) -> bool {
    let window = get_window();
    let Ok(Some(style)) = window.get_computed_style(element) else {
        return false;
    };

    let overflow = style.get_property_value("overflow").unwrap_or_default();
    let overflow_x = style.get_property_value("overflow-x").unwrap_or_default();
    let overflow_y = style.get_property_value("overflow-y").unwrap_or_default();

    let combined = format!("{overflow}{overflow_x}{overflow_y}");
    combined.contains("auto")
        || combined.contains("scroll")
        || combined.contains("hidden")
        || combined.contains("clip")
}

/// Check if the element is a containing block (has transforms, perspective, etc.).
#[allow(dead_code)]
pub(crate) fn is_containing_block(element: &Element) -> bool {
    let window = get_window();
    let Ok(Some(style)) = window.get_computed_style(element) else {
        return false;
    };

    let transform = style.get_property_value("transform").unwrap_or_default();
    let perspective = style.get_property_value("perspective").unwrap_or_default();
    let contain = style.get_property_value("contain").unwrap_or_default();
    let will_change = style.get_property_value("will-change").unwrap_or_default();
    let container_type = style
        .get_property_value("container-type")
        .unwrap_or_default();

    (transform != "none" && transform != "")
        || (perspective != "none" && perspective != "")
        || contain == "paint"
        || contain == "layout"
        || contain == "strict"
        || contain == "content"
        || will_change.contains("transform")
        || will_change.contains("perspective")
        || (container_type != "normal" && container_type != "")
}

/// Collect all overflow ancestors of the given element.
pub(crate) fn get_overflow_ancestors(element: &Element) -> Vec<Element> {
    let mut ancestors = Vec::new();
    let mut current = element.parent_element();

    while let Some(parent) = current {
        if is_overflow_element(&parent) {
            ancestors.push(parent.clone());
        }
        current = parent.parent_element();
    }

    // Always include the document element and window
    ancestors.push(get_document_element());
    ancestors
}

/// Find the CSS offset parent of the given element.
pub(crate) fn get_offset_parent(element: &Element) -> Option<Element> {
    // Try to cast to HtmlElement which has offsetParent
    if let Some(html_el) = element.dyn_ref::<HtmlElement>() {
        html_el.offset_parent()
    } else {
        None
    }
}

/// Find the containing block for the element.
#[allow(dead_code)]
pub(crate) fn get_containing_block(element: &Element) -> Option<Element> {
    let mut current = element.parent_element();

    while let Some(parent) = current {
        if is_containing_block(&parent) {
            return Some(parent);
        }
        current = parent.parent_element();
    }

    None
}

/// Get the viewport rect as a `ClippingRect`.
pub(crate) fn get_viewport_rect() -> ClippingRect {
    let window = get_window();
    let width = window
        .inner_width()
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let height = window
        .inner_height()
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);

    Box2D::new(Point2D::new(0.0, 0.0), Point2D::new(width, height))
}

/// Get the full document rect.
pub(crate) fn get_document_rect() -> LayoutRect {
    let doc_el = get_document_element();
    let html = doc_el.dyn_ref::<HtmlElement>();

    let width = html
        .map(|h| {
            let scroll_w = h.scroll_width() as f64;
            let client_w = h.client_width() as f64;
            scroll_w.max(client_w)
        })
        .unwrap_or(0.0);

    let height = html
        .map(|h| {
            let scroll_h = h.scroll_height() as f64;
            let client_h = h.client_height() as f64;
            scroll_h.max(client_h)
        })
        .unwrap_or(0.0);

    Rect::new(Point2D::new(0.0, 0.0), Size2D::new(width, height))
}

/// Convert a `DomRect` to a `LayoutRect`.
pub(crate) fn dom_rect_to_layout_rect(rect: &web_sys::DomRect) -> LayoutRect {
    Rect::new(
        Point2D::new(rect.x(), rect.y()),
        Size2D::new(rect.width(), rect.height()),
    )
}

/// Convert a `DomRect` to a `ClippingRect`.
pub(crate) fn dom_rect_to_clipping_rect(rect: &web_sys::DomRect) -> ClippingRect {
    Box2D::new(
        Point2D::new(rect.left(), rect.top()),
        Point2D::new(rect.right(), rect.bottom()),
    )
}

/// Get the scale of an element by comparing getBoundingClientRect to offset dimensions.
pub(crate) fn get_element_scale(element: &Element) -> (f64, f64) {
    let rect = element.get_bounding_client_rect();
    let html = element.dyn_ref::<HtmlElement>();

    let (offset_width, offset_height) = html
        .map(|h| (h.offset_width() as f64, h.offset_height() as f64))
        .unwrap_or((rect.width(), rect.height()));

    let scale_x = if offset_width > 0.0 {
        rect.width() / offset_width
    } else {
        1.0
    };

    let scale_y = if offset_height > 0.0 {
        rect.height() / offset_height
    } else {
        1.0
    };

    (scale_x, scale_y)
}

/// Check if the element has `direction: rtl`.
pub(crate) fn is_rtl(element: &Element) -> bool {
    let window = get_window();
    window
        .get_computed_style(element)
        .ok()
        .flatten()
        .and_then(|s| s.get_property_value("direction").ok())
        .map(|d| d == "rtl")
        .unwrap_or(false)
}

/// Compute the clipping rect by intersecting all overflow ancestors' visible rects.
pub(crate) fn compute_clipping_rect(
    _element: &Element,
    overflow_ancestors: &[Element],
) -> ClippingRect {
    let mut clip = get_viewport_rect();

    for ancestor in overflow_ancestors {
        // Skip the document element (it's the root, not a clipping ancestor)
        if *ancestor == get_document_element() {
            continue;
        }

        let ancestor_rect = dom_rect_to_clipping_rect(&ancestor.get_bounding_client_rect());

        // Intersect with current clip
        clip = Box2D::new(
            Point2D::new(
                clip.min.x.max(ancestor_rect.min.x),
                clip.min.y.max(ancestor_rect.min.y),
            ),
            Point2D::new(
                clip.max.x.min(ancestor_rect.max.x),
                clip.max.y.min(ancestor_rect.max.y),
            ),
        );
    }

    clip
}
