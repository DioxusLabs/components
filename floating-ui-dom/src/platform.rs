//! DOM platform implementation.

use euclid::{Point2D, Rect, Size2D};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use floating_ui_core::platform::Platform;
use floating_ui_core::types::{
    Boundary, ClientRect, ClippingRect, ElementRects, LayoutRect, RootBoundary, Size, Strategy,
};

use crate::utils;

/// Either a real DOM element or a virtual positioning reference.
pub enum ElementOrVirtual {
    /// A real DOM element.
    Element(web_sys::Element),
    /// A virtual element implementing custom bounding rect logic.
    Virtual(Box<dyn VirtualElement>),
}

impl ElementOrVirtual {
    /// Get the bounding client rect.
    pub fn get_bounding_client_rect(&self) -> ClientRect {
        match self {
            ElementOrVirtual::Element(el) => {
                let rect = el.get_bounding_client_rect();
                ClientRect::new(utils::dom_rect_to_layout_rect(&rect))
            }
            ElementOrVirtual::Virtual(v) => v.get_bounding_client_rect(),
        }
    }

    /// Get the context element (real DOM element for DOM traversal).
    pub fn context_element(&self) -> Option<&web_sys::Element> {
        match self {
            ElementOrVirtual::Element(el) => Some(el),
            ElementOrVirtual::Virtual(v) => v.context_element(),
        }
    }
}

impl From<web_sys::Element> for ElementOrVirtual {
    fn from(el: web_sys::Element) -> Self {
        ElementOrVirtual::Element(el)
    }
}

/// Trait for virtual elements used as positioning references.
///
/// Any object that can provide a bounding rect can serve as a reference element.
/// Examples: cursor positions, text selection ranges, or synthetic elements.
pub trait VirtualElement {
    /// The bounding client rect of this virtual element.
    fn get_bounding_client_rect(&self) -> ClientRect;

    /// An optional real DOM element for clipping ancestor traversal.
    fn context_element(&self) -> Option<&web_sys::Element> {
        None
    }
}

/// The DOM platform — implements [`Platform`] using `web-sys`.
pub struct DomPlatform;

impl Platform<ElementOrVirtual> for DomPlatform {
    fn get_element_rects(
        &self,
        reference: &ElementOrVirtual,
        floating: &ElementOrVirtual,
        strategy: Strategy,
    ) -> ElementRects {
        let ref_rect = reference.get_bounding_client_rect();
        let float_rect = floating.get_bounding_client_rect();

        // For absolute positioning, adjust for the offset parent
        let (offset_x, offset_y) = if strategy == Strategy::Absolute {
            if let Some(el) = floating.context_element() {
                if let Some(offset_parent) = utils::get_offset_parent(el) {
                    let parent_rect = offset_parent.get_bounding_client_rect();
                    let html = offset_parent.dyn_ref::<HtmlElement>();
                    let (border_left, border_top) = html
                        .map(|h| (h.client_left() as f64, h.client_top() as f64))
                        .unwrap_or((0.0, 0.0));
                    (parent_rect.x() + border_left, parent_rect.y() + border_top)
                } else {
                    (0.0, 0.0)
                }
            } else {
                (0.0, 0.0)
            }
        } else {
            (0.0, 0.0)
        };

        let reference_rect = LayoutRect::new(
            Point2D::new(ref_rect.left() - offset_x, ref_rect.top() - offset_y),
            Size2D::new(ref_rect.width(), ref_rect.height()),
        );

        let floating_rect = LayoutRect::new(
            Point2D::new(0.0, 0.0),
            Size2D::new(float_rect.width(), float_rect.height()),
        );

        ElementRects::new(reference_rect, floating_rect)
    }

    fn get_clipping_rect(
        &self,
        element: &ElementOrVirtual,
        boundary: &Boundary<ElementOrVirtual>,
        root_boundary: RootBoundary,
        _strategy: Strategy,
    ) -> ClippingRect {
        let context_el = element.context_element();

        match boundary {
            Boundary::ClippingAncestors => {
                if let Some(el) = context_el {
                    let ancestors = utils::get_overflow_ancestors(el);
                    let clip = utils::compute_clipping_rect(el, &ancestors);

                    // Intersect with root boundary
                    let root_clip = match root_boundary {
                        RootBoundary::Viewport => utils::get_viewport_rect(),
                        RootBoundary::Document => floating_ui_core::types::rect_to_clipping_rect(
                            utils::get_document_rect(),
                        ),
                        RootBoundary::Custom(rect) => rect,
                    };

                    euclid::Box2D::new(
                        Point2D::new(
                            clip.min.x.max(root_clip.min.x),
                            clip.min.y.max(root_clip.min.y),
                        ),
                        Point2D::new(
                            clip.max.x.min(root_clip.max.x),
                            clip.max.y.min(root_clip.max.y),
                        ),
                    )
                } else {
                    // No context element — use root boundary
                    match root_boundary {
                        RootBoundary::Viewport => utils::get_viewport_rect(),
                        RootBoundary::Document => floating_ui_core::types::rect_to_clipping_rect(
                            utils::get_document_rect(),
                        ),
                        RootBoundary::Custom(rect) => rect,
                    }
                }
            }
            Boundary::Element(el) => {
                let rect = el.get_bounding_client_rect();
                rect.to_box()
            }
            Boundary::Elements(els) => {
                if els.is_empty() {
                    return utils::get_viewport_rect();
                }
                let mut clip = els[0].get_bounding_client_rect().to_box();
                for el in &els[1..] {
                    let r = el.get_bounding_client_rect().to_box();
                    clip = euclid::Box2D::new(
                        Point2D::new(clip.min.x.max(r.min.x), clip.min.y.max(r.min.y)),
                        Point2D::new(clip.max.x.min(r.max.x), clip.max.y.min(r.max.y)),
                    );
                }
                clip
            }
        }
    }

    fn get_dimensions(&self, element: &ElementOrVirtual) -> Size {
        let rect = element.get_bounding_client_rect();
        Size2D::new(rect.width(), rect.height())
    }

    fn convert_offset_parent_relative_rect_to_viewport_relative_rect(
        &self,
        rect: LayoutRect,
        offset_parent: &ElementOrVirtual,
        _strategy: Strategy,
    ) -> LayoutRect {
        if let Some(el) = offset_parent.context_element() {
            let parent_rect = el.get_bounding_client_rect();
            let html = el.dyn_ref::<HtmlElement>();
            let (border_left, border_top) = html
                .map(|h| (h.client_left() as f64, h.client_top() as f64))
                .unwrap_or((0.0, 0.0));

            Rect::new(
                Point2D::new(
                    rect.origin.x + parent_rect.x() + border_left,
                    rect.origin.y + parent_rect.y() + border_top,
                ),
                rect.size,
            )
        } else {
            rect
        }
    }

    fn get_offset_parent(&self, element: &ElementOrVirtual) -> Option<ElementOrVirtual> {
        element
            .context_element()
            .and_then(utils::get_offset_parent)
            .map(ElementOrVirtual::Element)
    }

    fn is_element(&self, value: &ElementOrVirtual) -> bool {
        matches!(value, ElementOrVirtual::Element(_))
    }

    fn get_document_element(&self, _element: &ElementOrVirtual) -> Option<ElementOrVirtual> {
        Some(ElementOrVirtual::Element(utils::get_document_element()))
    }

    fn get_client_rects(&self, element: &ElementOrVirtual) -> Vec<ClientRect> {
        if let ElementOrVirtual::Element(el) = element {
            let rects = el.get_client_rects();
            let mut result = Vec::new();
            for i in 0..rects.length() {
                if let Some(rect) = rects.get(i) {
                    result.push(ClientRect::new(utils::dom_rect_to_layout_rect(&rect)));
                }
            }
            result
        } else {
            vec![element.get_bounding_client_rect()]
        }
    }

    fn is_rtl(&self, element: &ElementOrVirtual) -> bool {
        element
            .context_element()
            .map(utils::is_rtl)
            .unwrap_or(false)
    }

    fn get_scale(&self, element: &ElementOrVirtual) -> (f64, f64) {
        element
            .context_element()
            .map(utils::get_element_scale)
            .unwrap_or((1.0, 1.0))
    }
}
