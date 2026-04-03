//! The core positioning algorithm.
//!
//! [`compute_position`] is the entry point for all floating element positioning.
//! It computes initial coordinates from the placement, then runs the middleware
//! pipeline to refine them.

use crate::compute_coords::compute_coords_from_placement;
use crate::middleware::{Elements, Middleware, MiddlewareData, MiddlewareState, Reset};
use crate::platform::Platform;
use crate::types::{Placement, Point, Strategy};

/// Maximum number of pipeline resets before giving up.
const MAX_RESET_COUNT: u32 = 50;

// ---------------------------------------------------------------------------
// ComputePositionConfig
// ---------------------------------------------------------------------------

/// Configuration for [`compute_position`].
pub struct ComputePositionConfig<'a, E> {
    placement: Placement,
    strategy: Strategy,
    middleware: Vec<Box<dyn Middleware<E> + 'a>>,
    platform: &'a dyn Platform<E>,
}

impl<'a, E> ComputePositionConfig<'a, E> {
    /// Create a new config with default placement (`Bottom`) and strategy (`Absolute`).
    pub fn new(platform: &'a dyn Platform<E>) -> Self {
        Self {
            placement: Placement::default(),
            strategy: Strategy::default(),
            middleware: Vec::new(),
            platform,
        }
    }

    /// Set the desired placement.
    pub fn placement(mut self, p: Placement) -> Self {
        self.placement = p;
        self
    }

    /// Set the CSS positioning strategy.
    pub fn strategy(mut self, s: Strategy) -> Self {
        self.strategy = s;
        self
    }

    /// Set the middleware pipeline.
    pub fn middleware(mut self, m: Vec<Box<dyn Middleware<E> + 'a>>) -> Self {
        self.middleware = m;
        self
    }
}

// ---------------------------------------------------------------------------
// ComputePositionReturn
// ---------------------------------------------------------------------------

/// The result of [`compute_position`].
pub struct ComputePositionReturn {
    x: f64,
    y: f64,
    placement: Placement,
    strategy: Strategy,
    middleware_data: MiddlewareData,
}

impl ComputePositionReturn {
    /// The computed x coordinate for the floating element.
    pub fn x(&self) -> f64 {
        self.x
    }

    /// The computed y coordinate for the floating element.
    pub fn y(&self) -> f64 {
        self.y
    }

    /// The coordinates as a point.
    pub fn point(&self) -> Point {
        euclid::Point2D::new(self.x, self.y)
    }

    /// The final placement (may differ from the requested one if middleware like
    /// `flip` or `autoPlacement` changed it).
    pub fn placement(&self) -> Placement {
        self.placement
    }

    /// The CSS positioning strategy used.
    pub fn strategy(&self) -> Strategy {
        self.strategy
    }

    /// Data accumulated from all middleware in the pipeline.
    pub fn middleware_data(&self) -> &MiddlewareData {
        &self.middleware_data
    }
}

// ---------------------------------------------------------------------------
// compute_position
// ---------------------------------------------------------------------------

/// Compute the position of a floating element relative to a reference element.
///
/// This is the main entry point. It:
/// 1. Measures element rectangles via the platform.
/// 2. Computes initial coordinates from the placement.
/// 3. Runs the middleware pipeline, allowing each middleware to adjust the
///    coordinates, store data, or trigger a pipeline reset.
///
/// The pipeline supports up to 50 resets to prevent infinite loops from
/// conflicting middleware.
pub fn compute_position<E>(
    reference: &E,
    floating: &E,
    config: ComputePositionConfig<'_, E>,
) -> ComputePositionReturn {
    let platform = config.platform;
    let initial_placement = config.placement;
    let strategy = config.strategy;

    let rtl = platform.is_rtl(floating);

    let mut placement = initial_placement;
    let mut rects = platform.get_element_rects(reference, floating, strategy);
    let mut coords = compute_coords_from_placement(&rects, placement, rtl);
    let mut x = coords.x;
    let mut y = coords.y;
    let mut middleware_data = MiddlewareData::default();
    let mut reset_count: u32 = 0;

    // Middleware pipeline with reset support
    let mut i: usize = 0;
    while i < config.middleware.len() {
        let mw = &config.middleware[i];

        let state = MiddlewareState::new(
            x,
            y,
            initial_placement,
            placement,
            strategy,
            &middleware_data,
            &rects,
            Elements::new(reference, floating),
            platform,
        );

        let mut result = mw.compute(&state);

        // Apply coordinate modifications
        if let Some(new_x) = result.x() {
            x = new_x;
        }
        if let Some(new_y) = result.y() {
            y = new_y;
        }

        // Store middleware data
        if let Some(entry) = result.take_data() {
            middleware_data.apply_entry(entry);
        }

        // Handle reset
        if let Some(reset) = result.reset() {
            if reset_count < MAX_RESET_COUNT {
                reset_count += 1;

                match reset {
                    Reset::True => {
                        // Just restart the pipeline
                    }
                    Reset::WithPlacement(new_placement) => {
                        placement = *new_placement;
                    }
                    Reset::WithRects(new_rects) => {
                        rects = new_rects.clone();
                    }
                }

                // Recompute base coordinates
                coords = compute_coords_from_placement(&rects, placement, rtl);
                x = coords.x;
                y = coords.y;

                // Restart from the beginning
                i = 0;
                continue;
            }
        }

        i += 1;
    }

    ComputePositionReturn {
        x,
        y,
        placement,
        strategy,
        middleware_data,
    }
}
