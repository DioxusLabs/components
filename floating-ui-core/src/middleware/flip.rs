//! The flip middleware — changes placement when the floating element overflows.

use crate::detect_overflow::{detect_overflow, DetectOverflowOptions};
use crate::middleware::{
    FlipData, Middleware, MiddlewareDataEntry, MiddlewareReturn, MiddlewareState, Reset,
};
use crate::types::{get_side_overflow, Overflow, Placement, Side};

/// Strategy when all fallback placements overflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FallbackStrategy {
    /// Pick the placement with the least total overflow.
    BestFit,
    /// Revert to the initial placement.
    InitialPlacement,
}

impl Default for FallbackStrategy {
    fn default() -> Self {
        FallbackStrategy::BestFit
    }
}

/// Direction to try on the fallback axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FallbackAxisSideDirection {
    None,
    Start,
    End,
}

impl Default for FallbackAxisSideDirection {
    fn default() -> Self {
        FallbackAxisSideDirection::None
    }
}

/// Automatically flips the placement to avoid overflow.
///
/// When the floating element overflows its clipping boundary, this middleware
/// tries fallback placements in order and picks the best fit.
pub struct Flip<E: 'static> {
    main_axis: bool,
    cross_axis: bool,
    fallback_placements: Option<Vec<Placement>>,
    fallback_strategy: FallbackStrategy,
    fallback_axis_side_direction: FallbackAxisSideDirection,
    flip_alignment: bool,
    detect_overflow_options: DetectOverflowOptions<E>,
}

impl<E> Flip<E> {
    /// Create with defaults: flip on both axes, best-fit fallback.
    pub fn new() -> Self {
        Self {
            main_axis: true,
            cross_axis: true,
            fallback_placements: None,
            fallback_strategy: FallbackStrategy::default(),
            fallback_axis_side_direction: FallbackAxisSideDirection::default(),
            flip_alignment: true,
            detect_overflow_options: DetectOverflowOptions::new(),
        }
    }

    /// Whether to flip on the main axis (opposite side).
    pub fn main_axis(mut self, v: bool) -> Self {
        self.main_axis = v;
        self
    }

    /// Whether to flip on the cross axis (alignment).
    pub fn cross_axis(mut self, v: bool) -> Self {
        self.cross_axis = v;
        self
    }

    /// Override the list of fallback placements to try.
    pub fn fallback_placements(mut self, p: Vec<Placement>) -> Self {
        self.fallback_placements = Some(p);
        self
    }

    /// Strategy when all placements overflow.
    pub fn fallback_strategy(mut self, s: FallbackStrategy) -> Self {
        self.fallback_strategy = s;
        self
    }

    /// Direction to try on the cross axis.
    pub fn fallback_axis_side_direction(mut self, d: FallbackAxisSideDirection) -> Self {
        self.fallback_axis_side_direction = d;
        self
    }

    /// Whether to flip alignment variants too.
    pub fn flip_alignment(mut self, v: bool) -> Self {
        self.flip_alignment = v;
        self
    }

    /// Set overflow detection options.
    pub fn detect_overflow(mut self, opts: DetectOverflowOptions<E>) -> Self {
        self.detect_overflow_options = opts;
        self
    }
}

impl<E> Default for Flip<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E> Middleware<E> for Flip<E> {
    fn name(&self) -> &'static str {
        "flip"
    }

    fn compute(&self, state: &MiddlewareState<E>) -> MiddlewareReturn {
        let initial_placement = state.initial_placement();
        let placement = state.placement();

        // If the arrow middleware already applied an alignment offset,
        // don't flip (it would conflict).
        if let Some(arrow_data) = state.middleware_data().arrow() {
            if arrow_data.center_offset() != 0.0 {
                return MiddlewareReturn::empty();
            }
        }

        // Get the current flip state (index into fallback list)
        let flip_data = state.middleware_data().flip();
        let current_index = flip_data.map(|d| d.index()).unwrap_or(0);
        let mut overflows: Vec<(Placement, Overflow)> = flip_data
            .map(|d| d.overflows().to_vec())
            .unwrap_or_default();

        // Build the list of candidate placements
        let candidates = if let Some(ref fallbacks) = self.fallback_placements {
            let mut c = vec![initial_placement];
            c.extend(fallbacks.iter().copied());
            c
        } else {
            let mut c = vec![initial_placement, initial_placement.opposite()];

            // Add alignment variants if flip_alignment is enabled
            if self.flip_alignment {
                if let Some(_) = initial_placement.alignment() {
                    c.push(initial_placement.opposite_alignment());
                    c.push(initial_placement.opposite().opposite_alignment());
                }
            }

            // Add cross-axis placements
            if self.fallback_axis_side_direction != FallbackAxisSideDirection::None {
                let side = initial_placement.side();
                let cross_side = match side.axis() {
                    crate::types::Axis::Y => match self.fallback_axis_side_direction {
                        FallbackAxisSideDirection::Start => Side::Left,
                        FallbackAxisSideDirection::End => Side::Right,
                        FallbackAxisSideDirection::None => unreachable!(),
                    },
                    crate::types::Axis::X => match self.fallback_axis_side_direction {
                        FallbackAxisSideDirection::Start => Side::Top,
                        FallbackAxisSideDirection::End => Side::Bottom,
                        FallbackAxisSideDirection::None => unreachable!(),
                    },
                };
                let alignment = initial_placement.alignment();
                c.push(Placement::from_side_alignment(cross_side, alignment));
                c.push(Placement::from_side_alignment(
                    cross_side.opposite(),
                    alignment,
                ));
            }

            // Deduplicate
            let mut deduped = Vec::new();
            for p in c {
                if !deduped.contains(&p) {
                    deduped.push(p);
                }
            }
            deduped
        };

        // Detect overflow for the current placement
        let overflow = detect_overflow(state, &self.detect_overflow_options);

        // Check if we have overflow on the relevant axes
        let side = placement.side();
        let has_main_axis_overflow = if self.main_axis {
            let main_side_overflow = get_side_overflow(&overflow, side);
            main_side_overflow > 0.0
        } else {
            false
        };

        let has_cross_axis_overflow = if self.cross_axis {
            let cross_sides = match side.axis() {
                crate::types::Axis::Y => (overflow.left, overflow.right),
                crate::types::Axis::X => (overflow.top, overflow.bottom),
            };
            cross_sides.0 > 0.0 || cross_sides.1 > 0.0
        } else {
            false
        };

        // Store current overflow
        overflows.push((placement, overflow));

        // If we have overflow and there are more candidates to try
        let next_index = current_index + 1;
        if (has_main_axis_overflow || has_cross_axis_overflow) && next_index < candidates.len() {
            return MiddlewareReturn::empty()
                .with_data(MiddlewareDataEntry::Flip(FlipData::new(
                    next_index, overflows,
                )))
                .with_reset(Reset::WithPlacement(candidates[next_index]));
        }

        // All candidates exhausted — pick the best one
        if has_main_axis_overflow || has_cross_axis_overflow {
            let best_placement = match self.fallback_strategy {
                FallbackStrategy::BestFit => {
                    // Find the placement with the least total overflow
                    overflows
                        .iter()
                        .min_by(|(_, a), (_, b)| {
                            let total_a = a.top.max(0.0)
                                + a.right.max(0.0)
                                + a.bottom.max(0.0)
                                + a.left.max(0.0);
                            let total_b = b.top.max(0.0)
                                + b.right.max(0.0)
                                + b.bottom.max(0.0)
                                + b.left.max(0.0);
                            total_a
                                .partial_cmp(&total_b)
                                .unwrap_or(std::cmp::Ordering::Equal)
                        })
                        .map(|(p, _)| *p)
                        .unwrap_or(initial_placement)
                }
                FallbackStrategy::InitialPlacement => initial_placement,
            };

            if best_placement != placement {
                return MiddlewareReturn::empty()
                    .with_data(MiddlewareDataEntry::Flip(FlipData::new(
                        next_index, overflows,
                    )))
                    .with_reset(Reset::WithPlacement(best_placement));
            }
        }

        // No overflow or already at best — store data and continue
        MiddlewareReturn::empty().with_data(MiddlewareDataEntry::Flip(FlipData::new(
            next_index, overflows,
        )))
    }
}
