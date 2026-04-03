//! The middleware pipeline framework.
//!
//! Middleware are composable plugins that modify the positioning coordinates
//! computed by [`compute_position`](crate::compute_position). Each middleware
//! receives the current state and returns optional coordinate adjustments,
//! data to store, or a reset signal.

pub mod arrow;
pub mod auto_placement;
pub mod flip;
pub mod hide;
pub mod inline;
pub mod offset;
pub mod shift;
pub mod size;

use crate::platform::Platform;
use crate::types::{ElementRects, Overflow, Placement, Strategy};

// ---------------------------------------------------------------------------
// Elements
// ---------------------------------------------------------------------------

/// References to the positioning elements.
pub struct Elements<'a, E> {
    reference: &'a E,
    floating: &'a E,
}

impl<'a, E> Elements<'a, E> {
    /// Create a new `Elements`.
    pub fn new(reference: &'a E, floating: &'a E) -> Self {
        Self {
            reference,
            floating,
        }
    }

    /// The reference (anchor) element.
    pub fn reference(&self) -> &E {
        self.reference
    }

    /// The floating element being positioned.
    pub fn floating(&self) -> &E {
        self.floating
    }
}

// ---------------------------------------------------------------------------
// MiddlewareState
// ---------------------------------------------------------------------------

/// Read-only state passed to each middleware during the pipeline.
pub struct MiddlewareState<'a, E> {
    x: f64,
    y: f64,
    initial_placement: Placement,
    placement: Placement,
    strategy: Strategy,
    middleware_data: &'a MiddlewareData,
    rects: &'a ElementRects,
    elements: Elements<'a, E>,
    platform: &'a dyn Platform<E>,
}

impl<'a, E> MiddlewareState<'a, E> {
    /// Create a new middleware state. Typically called by `compute_position`.
    pub(crate) fn new(
        x: f64,
        y: f64,
        initial_placement: Placement,
        placement: Placement,
        strategy: Strategy,
        middleware_data: &'a MiddlewareData,
        rects: &'a ElementRects,
        elements: Elements<'a, E>,
        platform: &'a dyn Platform<E>,
    ) -> Self {
        Self {
            x,
            y,
            initial_placement,
            placement,
            strategy,
            middleware_data,
            rects,
            elements,
            platform,
        }
    }

    /// Current x coordinate.
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Current y coordinate.
    pub fn y(&self) -> f64 {
        self.y
    }

    /// The original placement from the config (never changes during the pipeline).
    pub fn initial_placement(&self) -> Placement {
        self.initial_placement
    }

    /// The current active placement (may change via flip/autoPlacement resets).
    pub fn placement(&self) -> Placement {
        self.placement
    }

    /// The CSS positioning strategy.
    pub fn strategy(&self) -> Strategy {
        self.strategy
    }

    /// Data accumulated from all middleware so far.
    pub fn middleware_data(&self) -> &MiddlewareData {
        self.middleware_data
    }

    /// The element bounding rectangles.
    pub fn rects(&self) -> &ElementRects {
        self.rects
    }

    /// The reference and floating elements.
    pub fn elements(&self) -> &Elements<'a, E> {
        &self.elements
    }

    /// The platform implementation.
    pub fn platform(&self) -> &dyn Platform<E> {
        self.platform
    }
}

// ---------------------------------------------------------------------------
// MiddlewareReturn
// ---------------------------------------------------------------------------

/// The result of a middleware computation.
pub struct MiddlewareReturn {
    x: Option<f64>,
    y: Option<f64>,
    data: Option<MiddlewareDataEntry>,
    reset: Option<Reset>,
}

impl MiddlewareReturn {
    /// Create an empty return (no modifications).
    pub fn empty() -> Self {
        Self {
            x: None,
            y: None,
            data: None,
            reset: None,
        }
    }

    /// Set the new x coordinate.
    pub fn with_x(mut self, x: f64) -> Self {
        self.x = Some(x);
        self
    }

    /// Set the new y coordinate.
    pub fn with_y(mut self, y: f64) -> Self {
        self.y = Some(y);
        self
    }

    /// Set both coordinates.
    pub fn with_coords(mut self, x: f64, y: f64) -> Self {
        self.x = Some(x);
        self.y = Some(y);
        self
    }

    /// Attach data to be stored in `MiddlewareData`.
    pub fn with_data(mut self, data: MiddlewareDataEntry) -> Self {
        self.data = Some(data);
        self
    }

    /// Request a pipeline reset.
    pub fn with_reset(mut self, reset: Reset) -> Self {
        self.reset = Some(reset);
        self
    }

    /// The new x coordinate, if any.
    pub fn x(&self) -> Option<f64> {
        self.x
    }

    /// The new y coordinate, if any.
    pub fn y(&self) -> Option<f64> {
        self.y
    }

    /// Data to store, if any.
    pub fn data(&self) -> Option<&MiddlewareDataEntry> {
        self.data.as_ref()
    }

    /// Take ownership of the data entry.
    pub(crate) fn take_data(&mut self) -> Option<MiddlewareDataEntry> {
        self.data.take()
    }

    /// Reset signal, if any.
    pub fn reset(&self) -> Option<&Reset> {
        self.reset.as_ref()
    }
}

// ---------------------------------------------------------------------------
// Reset
// ---------------------------------------------------------------------------

/// Signal to restart the middleware pipeline.
pub enum Reset {
    /// Re-run the pipeline with the current state.
    True,
    /// Re-run with a new placement.
    WithPlacement(Placement),
    /// Re-run with new element rects (e.g., after resizing the floating element).
    WithRects(ElementRects),
}

// ---------------------------------------------------------------------------
// Middleware trait
// ---------------------------------------------------------------------------

/// A positioning middleware that can modify coordinates during the pipeline.
pub trait Middleware<E> {
    /// A unique name for this middleware, used to key data in `MiddlewareData`.
    fn name(&self) -> &'static str;

    /// Compute coordinate modifications based on the current state.
    fn compute(&self, state: &MiddlewareState<E>) -> MiddlewareReturn;
}

// ---------------------------------------------------------------------------
// Middleware data types
// ---------------------------------------------------------------------------

/// Data produced by the offset middleware.
#[derive(Debug, Clone, Copy)]
pub struct OffsetData {
    x: f64,
    y: f64,
    placement: Placement,
}

impl OffsetData {
    /// Create new offset data.
    pub fn new(x: f64, y: f64, placement: Placement) -> Self {
        Self { x, y, placement }
    }

    /// The x offset applied.
    pub fn x(&self) -> f64 {
        self.x
    }

    /// The y offset applied.
    pub fn y(&self) -> f64 {
        self.y
    }

    /// The placement when the offset was computed.
    pub fn placement(&self) -> Placement {
        self.placement
    }
}

/// Data produced by the flip middleware.
#[derive(Debug, Clone)]
pub struct FlipData {
    index: usize,
    overflows: Vec<(Placement, Overflow)>,
}

impl FlipData {
    /// Create new flip data.
    pub fn new(index: usize, overflows: Vec<(Placement, Overflow)>) -> Self {
        Self { index, overflows }
    }

    /// The current index into the fallback placement list.
    pub fn index(&self) -> usize {
        self.index
    }

    /// The overflow values for each tested placement.
    pub fn overflows(&self) -> &[(Placement, Overflow)] {
        &self.overflows
    }
}

/// Data produced by the shift middleware.
#[derive(Debug, Clone, Copy)]
pub struct ShiftData {
    x: f64,
    y: f64,
}

impl ShiftData {
    /// Create new shift data.
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// The x shift applied.
    pub fn x(&self) -> f64 {
        self.x
    }

    /// The y shift applied.
    pub fn y(&self) -> f64 {
        self.y
    }
}

/// Data produced by the size middleware.
#[derive(Debug, Clone, Copy)]
pub struct SizeData {
    available_width: f64,
    available_height: f64,
}

impl SizeData {
    /// Create new size data.
    pub fn new(available_width: f64, available_height: f64) -> Self {
        Self {
            available_width,
            available_height,
        }
    }

    /// The available width before overflow.
    pub fn available_width(&self) -> f64 {
        self.available_width
    }

    /// The available height before overflow.
    pub fn available_height(&self) -> f64 {
        self.available_height
    }
}

/// Data produced by the arrow middleware.
#[derive(Debug, Clone, Copy)]
pub struct ArrowData {
    x: Option<f64>,
    y: Option<f64>,
    center_offset: f64,
}

impl ArrowData {
    /// Create new arrow data.
    pub fn new(x: Option<f64>, y: Option<f64>, center_offset: f64) -> Self {
        Self {
            x,
            y,
            center_offset,
        }
    }

    /// The x position of the arrow within the floating element.
    pub fn x(&self) -> Option<f64> {
        self.x
    }

    /// The y position of the arrow within the floating element.
    pub fn y(&self) -> Option<f64> {
        self.y
    }

    /// How far the arrow had to be shifted from the ideal center position.
    /// A non-zero value means the arrow couldn't point at the center of the reference.
    pub fn center_offset(&self) -> f64 {
        self.center_offset
    }
}

/// Data produced by the autoPlacement middleware.
#[derive(Debug, Clone)]
pub struct AutoPlacementData {
    index: usize,
    overflows: Vec<(Placement, Overflow)>,
}

impl AutoPlacementData {
    /// Create new auto-placement data.
    pub fn new(index: usize, overflows: Vec<(Placement, Overflow)>) -> Self {
        Self { index, overflows }
    }

    /// The current index into the placement list.
    pub fn index(&self) -> usize {
        self.index
    }

    /// The overflow values for each tested placement.
    pub fn overflows(&self) -> &[(Placement, Overflow)] {
        &self.overflows
    }
}

/// Data produced by the hide middleware.
#[derive(Debug, Clone, Copy)]
pub struct HideData {
    reference_hidden: bool,
    escaped: bool,
    reference_hidden_offsets: Option<Overflow>,
    escaped_offsets: Option<Overflow>,
}

impl HideData {
    /// Create new hide data.
    pub fn new(
        reference_hidden: bool,
        escaped: bool,
        reference_hidden_offsets: Option<Overflow>,
        escaped_offsets: Option<Overflow>,
    ) -> Self {
        Self {
            reference_hidden,
            escaped,
            reference_hidden_offsets,
            escaped_offsets,
        }
    }

    /// Whether the reference element is fully hidden by a clipping ancestor.
    pub fn reference_hidden(&self) -> bool {
        self.reference_hidden
    }

    /// Whether the floating element has escaped its reference's clipping boundary.
    pub fn escaped(&self) -> bool {
        self.escaped
    }

    /// The overflow values when checking reference visibility.
    pub fn reference_hidden_offsets(&self) -> Option<&Overflow> {
        self.reference_hidden_offsets.as_ref()
    }

    /// The overflow values when checking floating escape.
    pub fn escaped_offsets(&self) -> Option<&Overflow> {
        self.escaped_offsets.as_ref()
    }
}

/// Data produced by the inline middleware.
#[derive(Debug, Clone, Copy)]
pub struct InlineData {
    // The inline middleware mainly triggers a reset with custom rects.
    // Minimal data is stored.
}

impl InlineData {
    /// Create new inline data.
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for InlineData {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// MiddlewareDataEntry
// ---------------------------------------------------------------------------

/// Enum dispatch for middleware to return typed data.
pub enum MiddlewareDataEntry {
    Offset(OffsetData),
    Flip(FlipData),
    Shift(ShiftData),
    Size(SizeData),
    Arrow(ArrowData),
    AutoPlacement(AutoPlacementData),
    Hide(HideData),
    Inline(InlineData),
}

// ---------------------------------------------------------------------------
// MiddlewareData
// ---------------------------------------------------------------------------

/// Accumulated data from all middleware in the pipeline.
///
/// Each middleware stores its output in a typed field, avoiding `dyn Any` downcasting.
#[derive(Debug, Clone, Default)]
pub struct MiddlewareData {
    offset: Option<OffsetData>,
    flip: Option<FlipData>,
    shift: Option<ShiftData>,
    size: Option<SizeData>,
    arrow: Option<ArrowData>,
    auto_placement: Option<AutoPlacementData>,
    hide: Option<HideData>,
    inline: Option<InlineData>,
}

impl MiddlewareData {
    /// Offset middleware data.
    pub fn offset(&self) -> Option<&OffsetData> {
        self.offset.as_ref()
    }

    /// Flip middleware data.
    pub fn flip(&self) -> Option<&FlipData> {
        self.flip.as_ref()
    }

    /// Shift middleware data.
    pub fn shift(&self) -> Option<&ShiftData> {
        self.shift.as_ref()
    }

    /// Size middleware data.
    pub fn size(&self) -> Option<&SizeData> {
        self.size.as_ref()
    }

    /// Arrow middleware data.
    pub fn arrow(&self) -> Option<&ArrowData> {
        self.arrow.as_ref()
    }

    /// Auto-placement middleware data.
    pub fn auto_placement(&self) -> Option<&AutoPlacementData> {
        self.auto_placement.as_ref()
    }

    /// Hide middleware data.
    pub fn hide(&self) -> Option<&HideData> {
        self.hide.as_ref()
    }

    /// Inline middleware data.
    pub fn inline(&self) -> Option<&InlineData> {
        self.inline.as_ref()
    }

    /// Apply a data entry from a middleware return.
    pub(crate) fn apply_entry(&mut self, entry: MiddlewareDataEntry) {
        match entry {
            MiddlewareDataEntry::Offset(d) => self.offset = Some(d),
            MiddlewareDataEntry::Flip(d) => self.flip = Some(d),
            MiddlewareDataEntry::Shift(d) => self.shift = Some(d),
            MiddlewareDataEntry::Size(d) => self.size = Some(d),
            MiddlewareDataEntry::Arrow(d) => self.arrow = Some(d),
            MiddlewareDataEntry::AutoPlacement(d) => self.auto_placement = Some(d),
            MiddlewareDataEntry::Hide(d) => self.hide = Some(d),
            MiddlewareDataEntry::Inline(d) => self.inline = Some(d),
        }
    }
}
