//! Defines the [`Icon`] component and its sub-components.
use dioxus::prelude::*;
use std::fmt;

/// A [`ViewBox`] represents a rectangular region defined by its top-left corner position and dimensions
///
/// min_x and min_y represent the smallest X and Y coordinates that the `ViewBox` may have
/// (the origin coordinates of the `ViewBox`) and the width and height specify the `ViewBox` size.
///
/// The resulting `ViewBox` is a rectangle in user space mapped to the bounds of the viewport of an SVG element
/// (not the browser viewport)
#[derive(Copy, Clone, PartialEq)]
pub struct ViewBox {
    /// The x coordinate of the top left corner
    pub min_x: u16,
    /// The y coordinate of the top left corner
    pub min_y: u16,
    /// The width of the `ViewBox`
    pub width: u16,
    /// The height of the `ViewBox`
    pub height: u16,
}

impl ViewBox {
    /// Creates a new `ViewBox`, with width and height at top-left(x, y) position
    pub fn new(min_x: u16, min_y: u16, width: u16, height: u16) -> Self {
        Self {
            min_x,
            min_y,
            width,
            height,
        }
    }
}

impl fmt::Display for ViewBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.min_x, self.min_y, self.width, self.height
        )
    }
}

/// Shape types to be used at the end of open sub-paths
#[derive(Copy, Clone, PartialEq, Default)]
#[allow(dead_code)]
pub enum LineCap {
    /// The stroke for each sub-path does not extend beyond its two endpoints.
    /// On a zero length sub-path, the path will not be rendered at all.
    Butt,
    /// The stroke will be extended by a half circle with a diameter equal to the stroke width.
    /// On a zero length sub-path, the stroke consists of a full circle centered at the sub-path's point.
    #[default]
    Round,
    /// The stroke will be extended by a rectangle with a width equal to half the width of the stroke and
    /// a height equal to the width of the stroke.
    /// On a zero length sub-path, the stroke consists of a square with its width equal to the stroke width,
    /// centered at the sub-path's point.
    Square,
}

impl LineCap {
    fn to_class(self) -> &'static str {
        match self {
            LineCap::Butt => "butt",
            LineCap::Round => "round",
            LineCap::Square => "square",
        }
    }
}

/// Shape types to be used to join path segments
#[derive(Copy, Clone, PartialEq, Default)]
#[allow(dead_code)]
pub enum LineJoin {
    /// The arcs shape is formed by extending the outer edges of the stroke at the join point with arcs
    /// that have the same curvature as the outer edges at the join point
    Arcs,
    /// Bevelled corner
    Bevel,
    /// The corner is formed by extending the outer edges of the stroke at the tangents of the path segments
    /// until they intersect
    Miter,
    /// This provides a better rendering than `Miter` on very sharp join but isn't widely supported yet
    MiterClip,
    /// Round corner
    #[default]
    Round,
}

impl LineJoin {
    fn to_class(self) -> &'static str {
        match self {
            LineJoin::Arcs => "arcs",
            LineJoin::Bevel => "bevel",
            LineJoin::Miter => "miter",
            LineJoin::MiterClip => "miter-clip",
            LineJoin::Round => "round",
        }
    }
}

/// The props for the [`Icon`] component
#[derive(Props, Clone, PartialEq)]
pub struct IconProps {
    /// The position and dimension, in user space, of an SVG viewport
    /// A transform stretches or resizes the SVG viewport to fit a particular container element
    #[props(default = ViewBox::new(0, 0, 24, 24))]
    pub view_box: ViewBox,

    /// The horizontal length in the user coordinate system
    #[props(default = 24)]
    pub width: u16,

    /// The vertical length in the user coordinate system
    #[props(default = 24)]
    pub height: u16,

    /// The width of the stroke to be applied to the shape
    #[props(default = 2)]
    pub stroke_width: u8,

    /// The shape to be used at the end of open sub-paths when they are stroked
    #[props(default)]
    pub stroke_line_cap: LineCap,

    /// The shape to be used at the corners of paths when they are stroked
    #[props(default)]
    pub stroke_line_join: LineJoin,

    /// The color used to paint the outline of the shape.
    #[props(default = "var(--secondary-color-4)")]
    pub stroke: &'static str,

    /// The color used to paint the element.
    #[props(default = "none")]
    pub fill: &'static str,

    /// Additional attributes to apply to the svg element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The content of the svg element
    pub children: Element,
}

/// # Icon
///
/// The `Icon` component is used to render the SVG elements
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::icon::Icon;
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Icon {
///             circle { cx: 12, cy:12, r: 10 }
///             path { d: "M8 14s1.5 2 4 2 4-2 4-2" }
///             line { x1: 9, x2: 9.01, y1: 9, y2: 9 }
///             line { x1: 15, x2: 15.01, y1: 9, y2: 9 }
///         }
///     }
/// }
/// ```
#[component]
pub fn Icon(props: IconProps) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: props.view_box.to_string(),
            width: props.width,
            height: props.height,
            fill: props.fill,
            stroke: props.stroke,
            stroke_linecap: props.stroke_line_cap.to_class(),
            stroke_linejoin: props.stroke_line_join.to_class(),
            stroke_width: props.stroke_width,
            ..props.attributes,
            {props.children}
        }
    }
}
