//! Core types for the floating-ui positioning engine.
//!
//! Uses [`euclid`] for geometric primitives with custom unit types
//! for compile-time coordinate space safety.

use euclid::{Box2D, Point2D, Rect, SideOffsets2D, Size2D};

// ---------------------------------------------------------------------------
// Unit types
// ---------------------------------------------------------------------------

/// Coordinate space relative to the floating element's offset parent.
pub struct OffsetParentSpace;

/// Coordinate space relative to the viewport.
pub struct ViewportSpace;

// ---------------------------------------------------------------------------
// Euclid type aliases
// ---------------------------------------------------------------------------

/// A 2D point in viewport space.
pub type Point = Point2D<f64, ViewportSpace>;

/// A 2D size (width × height) in viewport space.
pub type Size = Size2D<f64, ViewportSpace>;

/// An axis-aligned rectangle with origin + size, used for element layout bounds.
pub type LayoutRect = Rect<f64, ViewportSpace>;

/// An axis-aligned rectangle defined by min/max corners, used for clipping regions.
///
/// `Box2D` is preferred for clipping because it supports efficient intersection
/// and containment queries.
pub type ClippingRect = Box2D<f64, ViewportSpace>;

/// Per-side overflow values. Positive means overflow, negative means remaining space.
pub type Overflow = SideOffsets2D<f64, ViewportSpace>;

/// Per-side padding values.
pub type Padding = SideOffsets2D<f64, ViewportSpace>;

// ---------------------------------------------------------------------------
// ElementRects
// ---------------------------------------------------------------------------

/// The bounding rectangles of the reference and floating elements.
#[derive(Debug, Clone, Copy)]
pub struct ElementRects {
    reference: LayoutRect,
    floating: LayoutRect,
}

impl ElementRects {
    /// Create a new `ElementRects` from the reference and floating layout rects.
    pub fn new(reference: LayoutRect, floating: LayoutRect) -> Self {
        Self {
            reference,
            floating,
        }
    }

    /// The reference element's bounding rectangle.
    pub fn reference(&self) -> LayoutRect {
        self.reference
    }

    /// The floating element's bounding rectangle.
    pub fn floating(&self) -> LayoutRect {
        self.floating
    }
}

// ---------------------------------------------------------------------------
// ClientRect
// ---------------------------------------------------------------------------

/// A layout rectangle with convenient edge accessors, used for inline element
/// client rects (analogous to `DOMRect`).
#[derive(Debug, Clone, Copy)]
pub struct ClientRect {
    rect: LayoutRect,
}

impl ClientRect {
    /// Create from a layout rect.
    pub fn new(rect: LayoutRect) -> Self {
        Self { rect }
    }

    /// Create from individual coordinates.
    pub fn from_edges(top: f64, right: f64, bottom: f64, left: f64) -> Self {
        Self {
            rect: LayoutRect::new(
                Point2D::new(left, top),
                Size2D::new(right - left, bottom - top),
            ),
        }
    }

    /// The underlying layout rect.
    pub fn rect(&self) -> LayoutRect {
        self.rect
    }

    /// Top edge (minimum y).
    pub fn top(&self) -> f64 {
        self.rect.min_y()
    }

    /// Right edge (maximum x).
    pub fn right(&self) -> f64 {
        self.rect.max_x()
    }

    /// Bottom edge (maximum y).
    pub fn bottom(&self) -> f64 {
        self.rect.max_y()
    }

    /// Left edge (minimum x).
    pub fn left(&self) -> f64 {
        self.rect.min_x()
    }

    /// Width.
    pub fn width(&self) -> f64 {
        self.rect.width()
    }

    /// Height.
    pub fn height(&self) -> f64 {
        self.rect.height()
    }

    /// Convert to a `ClippingRect` (`Box2D`).
    pub fn to_box(&self) -> ClippingRect {
        Box2D::new(
            Point2D::new(self.left(), self.top()),
            Point2D::new(self.right(), self.bottom()),
        )
    }
}

// ---------------------------------------------------------------------------
// Side
// ---------------------------------------------------------------------------

/// One of the four sides of a rectangle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

impl Side {
    /// The axis this side lies on.
    pub fn axis(&self) -> Axis {
        match self {
            Side::Top | Side::Bottom => Axis::Y,
            Side::Left | Side::Right => Axis::X,
        }
    }

    /// The opposite side.
    pub fn opposite(&self) -> Side {
        match self {
            Side::Top => Side::Bottom,
            Side::Bottom => Side::Top,
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }

    /// Whether this side is vertical (top or bottom).
    pub fn is_vertical(&self) -> bool {
        matches!(self, Side::Top | Side::Bottom)
    }

    /// Whether this side is horizontal (left or right).
    pub fn is_horizontal(&self) -> bool {
        !self.is_vertical()
    }
}

// ---------------------------------------------------------------------------
// Alignment
// ---------------------------------------------------------------------------

/// Alignment along the cross axis of a placement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Alignment {
    Start,
    End,
}

impl Alignment {
    /// The opposite alignment.
    pub fn opposite(&self) -> Alignment {
        match self {
            Alignment::Start => Alignment::End,
            Alignment::End => Alignment::Start,
        }
    }
}

// ---------------------------------------------------------------------------
// Placement (12 variants: 4 sides × 3 alignments)
// ---------------------------------------------------------------------------

/// Where to position the floating element relative to its reference.
///
/// There are 12 placements: each of the 4 sides, optionally combined with
/// `Start` or `End` alignment along the cross axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Placement {
    Top,
    TopStart,
    TopEnd,
    Bottom,
    BottomStart,
    BottomEnd,
    Left,
    LeftStart,
    LeftEnd,
    Right,
    RightStart,
    RightEnd,
}

impl Placement {
    /// All 12 placements.
    pub const ALL: [Placement; 12] = [
        Placement::Top,
        Placement::TopStart,
        Placement::TopEnd,
        Placement::Bottom,
        Placement::BottomStart,
        Placement::BottomEnd,
        Placement::Left,
        Placement::LeftStart,
        Placement::LeftEnd,
        Placement::Right,
        Placement::RightStart,
        Placement::RightEnd,
    ];

    /// The side this placement is on.
    pub fn side(&self) -> Side {
        match self {
            Placement::Top | Placement::TopStart | Placement::TopEnd => Side::Top,
            Placement::Bottom | Placement::BottomStart | Placement::BottomEnd => Side::Bottom,
            Placement::Left | Placement::LeftStart | Placement::LeftEnd => Side::Left,
            Placement::Right | Placement::RightStart | Placement::RightEnd => Side::Right,
        }
    }

    /// The alignment, if any.
    pub fn alignment(&self) -> Option<Alignment> {
        match self {
            Placement::TopStart
            | Placement::BottomStart
            | Placement::LeftStart
            | Placement::RightStart => Some(Alignment::Start),
            Placement::TopEnd | Placement::BottomEnd | Placement::LeftEnd | Placement::RightEnd => {
                Some(Alignment::End)
            }
            _ => None,
        }
    }

    /// The placement on the opposite side with the same alignment.
    pub fn opposite(&self) -> Placement {
        match self {
            Placement::Top => Placement::Bottom,
            Placement::TopStart => Placement::BottomStart,
            Placement::TopEnd => Placement::BottomEnd,
            Placement::Bottom => Placement::Top,
            Placement::BottomStart => Placement::TopStart,
            Placement::BottomEnd => Placement::TopEnd,
            Placement::Left => Placement::Right,
            Placement::LeftStart => Placement::RightStart,
            Placement::LeftEnd => Placement::RightEnd,
            Placement::Right => Placement::Left,
            Placement::RightStart => Placement::LeftStart,
            Placement::RightEnd => Placement::LeftEnd,
        }
    }

    /// The same side with the opposite alignment. Unaligned placements return themselves.
    pub fn opposite_alignment(&self) -> Placement {
        match self {
            Placement::TopStart => Placement::TopEnd,
            Placement::TopEnd => Placement::TopStart,
            Placement::BottomStart => Placement::BottomEnd,
            Placement::BottomEnd => Placement::BottomStart,
            Placement::LeftStart => Placement::LeftEnd,
            Placement::LeftEnd => Placement::LeftStart,
            Placement::RightStart => Placement::RightEnd,
            Placement::RightEnd => Placement::RightStart,
            other => *other,
        }
    }

    /// Build a placement from a side and optional alignment.
    pub fn from_side_alignment(side: Side, alignment: Option<Alignment>) -> Placement {
        match (side, alignment) {
            (Side::Top, None) => Placement::Top,
            (Side::Top, Some(Alignment::Start)) => Placement::TopStart,
            (Side::Top, Some(Alignment::End)) => Placement::TopEnd,
            (Side::Bottom, None) => Placement::Bottom,
            (Side::Bottom, Some(Alignment::Start)) => Placement::BottomStart,
            (Side::Bottom, Some(Alignment::End)) => Placement::BottomEnd,
            (Side::Left, None) => Placement::Left,
            (Side::Left, Some(Alignment::Start)) => Placement::LeftStart,
            (Side::Left, Some(Alignment::End)) => Placement::LeftEnd,
            (Side::Right, None) => Placement::Right,
            (Side::Right, Some(Alignment::Start)) => Placement::RightStart,
            (Side::Right, Some(Alignment::End)) => Placement::RightEnd,
        }
    }
}

impl Default for Placement {
    fn default() -> Self {
        Placement::Bottom
    }
}

// ---------------------------------------------------------------------------
// Strategy
// ---------------------------------------------------------------------------

/// CSS positioning strategy for the floating element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Strategy {
    /// `position: absolute` — positioned relative to the nearest positioned ancestor.
    #[default]
    Absolute,
    /// `position: fixed` — positioned relative to the viewport.
    Fixed,
}

// ---------------------------------------------------------------------------
// Axis
// ---------------------------------------------------------------------------

/// A geometric axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Axis {
    X,
    Y,
}

impl Axis {
    /// The perpendicular axis.
    pub fn opposite(&self) -> Axis {
        match self {
            Axis::X => Axis::Y,
            Axis::Y => Axis::X,
        }
    }
}

// ---------------------------------------------------------------------------
// Boundary / overflow detection configuration
// ---------------------------------------------------------------------------

/// What to use as the clipping boundary for overflow detection.
pub enum Boundary<E> {
    /// Use all clipping ancestors of the element.
    ClippingAncestors,
    /// A single element as the boundary.
    Element(E),
    /// Multiple elements whose union forms the boundary.
    Elements(Vec<E>),
}

impl<E> Default for Boundary<E> {
    fn default() -> Self {
        Boundary::ClippingAncestors
    }
}

/// The root boundary to intersect with the element boundary.
#[derive(Debug, Clone, Copy)]
pub enum RootBoundary {
    /// The browser viewport.
    Viewport,
    /// The full document.
    Document,
    /// A custom rectangle.
    Custom(ClippingRect),
}

impl Default for RootBoundary {
    fn default() -> Self {
        RootBoundary::Viewport
    }
}

/// Which element to check for overflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ElementContext {
    /// Check overflow of the floating element (default).
    #[default]
    Floating,
    /// Check overflow of the reference element.
    Reference,
}

// ---------------------------------------------------------------------------
// PaddingInput
// ---------------------------------------------------------------------------

/// User-facing padding specification, convertible to a [`Padding`] (`SideOffsets2D`).
#[derive(Debug, Clone, Copy)]
pub enum PaddingInput {
    /// Same padding on all sides.
    All(f64),
    /// Different padding per side.
    PerSide {
        top: f64,
        right: f64,
        bottom: f64,
        left: f64,
    },
}

impl PaddingInput {
    /// Convert to a `Padding` value.
    pub fn to_padding(&self) -> Padding {
        match self {
            PaddingInput::All(v) => SideOffsets2D::new_all_same(*v),
            PaddingInput::PerSide {
                top,
                right,
                bottom,
                left,
            } => SideOffsets2D::new(*top, *right, *bottom, *left),
        }
    }
}

impl Default for PaddingInput {
    fn default() -> Self {
        PaddingInput::All(0.0)
    }
}

impl From<f64> for PaddingInput {
    fn from(v: f64) -> Self {
        PaddingInput::All(v)
    }
}

// ---------------------------------------------------------------------------
// Helper functions
// ---------------------------------------------------------------------------

/// Convert a `LayoutRect` to a `ClippingRect` (`Box2D`).
pub fn rect_to_clipping_rect(rect: LayoutRect) -> ClippingRect {
    Box2D::new(
        Point2D::new(rect.min_x(), rect.min_y()),
        Point2D::new(rect.max_x(), rect.max_y()),
    )
}

/// Convert a `ClippingRect` (`Box2D`) to a `LayoutRect`.
pub fn clipping_rect_to_rect(clip: ClippingRect) -> LayoutRect {
    LayoutRect::new(clip.min, Size2D::new(clip.width(), clip.height()))
}

/// Extract a side value from an `Overflow` / `SideOffsets2D`.
pub fn get_side_overflow(overflow: &Overflow, side: Side) -> f64 {
    match side {
        Side::Top => overflow.top,
        Side::Right => overflow.right,
        Side::Bottom => overflow.bottom,
        Side::Left => overflow.left,
    }
}

/// Get the length of a rect along an axis.
pub fn rect_axis_length(rect: &LayoutRect, axis: Axis) -> f64 {
    match axis {
        Axis::X => rect.width(),
        Axis::Y => rect.height(),
    }
}

/// Get the origin coordinate of a rect along an axis.
pub fn rect_axis_origin(rect: &LayoutRect, axis: Axis) -> f64 {
    match axis {
        Axis::X => rect.origin.x,
        Axis::Y => rect.origin.y,
    }
}
