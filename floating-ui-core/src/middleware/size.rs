//! The size middleware — constrains floating element dimensions to available space.

use crate::detect_overflow::{detect_overflow, DetectOverflowOptions};
use crate::middleware::{
    Middleware, MiddlewareDataEntry, MiddlewareReturn, MiddlewareState, Reset, SizeData,
};
use crate::types::{ElementRects, Placement, Side};

/// Information about available space, passed to the `apply` callback.
pub struct AvailableSize {
    available_width: f64,
    available_height: f64,
    rects: ElementRects,
    placement: Placement,
}

impl AvailableSize {
    /// The maximum width before overflow.
    pub fn available_width(&self) -> f64 {
        self.available_width
    }

    /// The maximum height before overflow.
    pub fn available_height(&self) -> f64 {
        self.available_height
    }

    /// The current element rects.
    pub fn rects(&self) -> &ElementRects {
        &self.rects
    }

    /// The current placement.
    pub fn placement(&self) -> Placement {
        self.placement
    }
}

/// Constrains the floating element's dimensions based on available space.
///
/// The `apply` callback receives available width/height and should set
/// `max-width`/`max-height` styles on the floating element. After applying,
/// the middleware triggers a reset to re-measure the now-resized element.
pub struct Size<E: 'static> {
    apply: Option<Box<dyn Fn(&AvailableSize)>>,
    detect_overflow_options: DetectOverflowOptions<E>,
}

impl<E> Size<E> {
    /// Create with the given apply callback.
    pub fn new(apply: impl Fn(&AvailableSize) + 'static) -> Self {
        Self {
            apply: Some(Box::new(apply)),
            detect_overflow_options: DetectOverflowOptions::new(),
        }
    }

    /// Create without an apply callback (data-only mode).
    pub fn data_only() -> Self {
        Self {
            apply: None,
            detect_overflow_options: DetectOverflowOptions::new(),
        }
    }

    /// Set overflow detection options.
    pub fn detect_overflow(mut self, opts: DetectOverflowOptions<E>) -> Self {
        self.detect_overflow_options = opts;
        self
    }
}

impl<E> Middleware<E> for Size<E> {
    fn name(&self) -> &'static str {
        "size"
    }

    fn compute(&self, state: &MiddlewareState<E>) -> MiddlewareReturn {
        let placement = state.placement();
        let side = placement.side();
        let rects = state.rects();

        let overflow = detect_overflow(state, &self.detect_overflow_options);

        // Calculate available dimensions
        let floating = rects.floating();

        // Max clipping dimensions: full dimensions minus overflow on each side
        let max_width = floating.size.width - overflow.left.max(0.0) - overflow.right.max(0.0);
        let max_height = floating.size.height - overflow.top.max(0.0) - overflow.bottom.max(0.0);

        // Available space based on the side
        let available_height = match side {
            Side::Top => {
                let space = overflow.top;
                floating.size.height - space.max(0.0)
            }
            Side::Bottom => {
                let space = overflow.bottom;
                floating.size.height - space.max(0.0)
            }
            _ => max_height,
        };

        let available_width = match side {
            Side::Left => {
                let space = overflow.left;
                floating.size.width - space.max(0.0)
            }
            Side::Right => {
                let space = overflow.right;
                floating.size.width - space.max(0.0)
            }
            _ => max_width,
        };

        let info = AvailableSize {
            available_width,
            available_height,
            rects: *rects,
            placement,
        };

        // Call the apply callback
        if let Some(ref apply) = self.apply {
            apply(&info);
        }

        let data = SizeData::new(available_width, available_height);

        // Check if we've already reset for size (avoid infinite loops)
        if state.middleware_data().size().is_some() {
            // Already reset once — just return data
            return MiddlewareReturn::empty().with_data(MiddlewareDataEntry::Size(data));
        }

        // Trigger a reset to re-measure after the apply callback has resized
        // the floating element
        if self.apply.is_some() {
            let new_rects = state.platform().get_element_rects(
                state.elements().reference(),
                state.elements().floating(),
                state.strategy(),
            );

            // Only reset if dimensions actually changed
            let old_floating = rects.floating();
            let new_floating = new_rects.floating();
            if (new_floating.size.width - old_floating.size.width).abs() > 0.5
                || (new_floating.size.height - old_floating.size.height).abs() > 0.5
            {
                return MiddlewareReturn::empty()
                    .with_data(MiddlewareDataEntry::Size(data))
                    .with_reset(Reset::WithRects(new_rects));
            }
        }

        MiddlewareReturn::empty().with_data(MiddlewareDataEntry::Size(data))
    }
}
