//! The auto-placement middleware — automatically selects the best placement.

use crate::detect_overflow::{detect_overflow, DetectOverflowOptions};
use crate::middleware::{
    AutoPlacementData, Middleware, MiddlewareDataEntry, MiddlewareReturn, MiddlewareState, Reset,
};
use crate::types::{get_side_overflow, Alignment, Overflow, Placement};

/// Automatically selects the placement with the most available space.
///
/// Unlike [`Flip`](super::flip::Flip), which uses a fallback strategy,
/// auto-placement scores all allowed placements and picks the best.
///
/// **Cannot be used alongside `Flip`** — their strategies conflict and would
/// cause infinite reset loops.
pub struct AutoPlacement<E: 'static> {
    cross_axis: bool,
    alignment: Option<Alignment>,
    auto_alignment: bool,
    allowed_placements: Option<Vec<Placement>>,
    detect_overflow_options: DetectOverflowOptions<E>,
}

impl<E> AutoPlacement<E> {
    /// Create with defaults: auto-alignment enabled, all placements allowed.
    pub fn new() -> Self {
        Self {
            cross_axis: false,
            alignment: None,
            auto_alignment: true,
            allowed_placements: None,
            detect_overflow_options: DetectOverflowOptions::new(),
        }
    }

    /// Whether to consider cross-axis overflow when scoring placements.
    pub fn cross_axis(mut self, v: bool) -> Self {
        self.cross_axis = v;
        self
    }

    /// Preferred alignment for aligned placements.
    pub fn alignment(mut self, a: Alignment) -> Self {
        self.alignment = Some(a);
        self
    }

    /// Whether to automatically try both alignments (default: true).
    pub fn auto_alignment(mut self, v: bool) -> Self {
        self.auto_alignment = v;
        self
    }

    /// Restrict the set of placements to consider.
    pub fn allowed_placements(mut self, p: Vec<Placement>) -> Self {
        self.allowed_placements = Some(p);
        self
    }

    /// Set overflow detection options.
    pub fn detect_overflow(mut self, opts: DetectOverflowOptions<E>) -> Self {
        self.detect_overflow_options = opts;
        self
    }
}

impl<E> Default for AutoPlacement<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E> Middleware<E> for AutoPlacement<E> {
    fn name(&self) -> &'static str {
        "autoPlacement"
    }

    fn compute(&self, state: &MiddlewareState<E>) -> MiddlewareReturn {
        let placement = state.placement();

        // Get existing auto-placement data
        let ap_data = state.middleware_data().auto_placement();
        let current_index = ap_data.map(|d| d.index()).unwrap_or(0);
        let mut overflows: Vec<(Placement, Overflow)> =
            ap_data.map(|d| d.overflows().to_vec()).unwrap_or_default();

        // Build the list of placements to evaluate
        let candidates = if let Some(ref allowed) = self.allowed_placements {
            allowed.clone()
        } else {
            let mut all: Vec<Placement> = Placement::ALL.to_vec();

            // Filter by alignment preference
            if let Some(align) = self.alignment {
                all.retain(|p| {
                    p.alignment() == Some(align) || (self.auto_alignment && p.alignment().is_some())
                });
            }

            all
        };

        if candidates.is_empty() {
            return MiddlewareReturn::empty();
        }

        // Detect overflow for current placement
        let overflow = detect_overflow(state, &self.detect_overflow_options);
        overflows.push((placement, overflow));

        // Check if there are more candidates to try
        let next_index = current_index + 1;
        if next_index < candidates.len() {
            // Check if current placement has overflow
            let side = placement.side();
            let main_overflow = get_side_overflow(&overflow, side);
            let has_overflow = main_overflow > 0.0
                || (self.cross_axis && {
                    let cross_sides = match side.axis() {
                        crate::types::Axis::Y => (overflow.left, overflow.right),
                        crate::types::Axis::X => (overflow.top, overflow.bottom),
                    };
                    cross_sides.0 > 0.0 || cross_sides.1 > 0.0
                });

            if has_overflow {
                return MiddlewareReturn::empty()
                    .with_data(MiddlewareDataEntry::AutoPlacement(AutoPlacementData::new(
                        next_index, overflows,
                    )))
                    .with_reset(Reset::WithPlacement(candidates[next_index]));
            }
        }

        // All candidates tested — pick the one with least overflow
        let best = overflows
            .iter()
            .min_by(|(_, a), (_, b)| {
                let total_a =
                    a.top.max(0.0) + a.right.max(0.0) + a.bottom.max(0.0) + a.left.max(0.0);
                let total_b =
                    b.top.max(0.0) + b.right.max(0.0) + b.bottom.max(0.0) + b.left.max(0.0);
                total_a
                    .partial_cmp(&total_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(p, _)| *p)
            .unwrap_or(placement);

        if best != placement {
            return MiddlewareReturn::empty()
                .with_data(MiddlewareDataEntry::AutoPlacement(AutoPlacementData::new(
                    next_index, overflows,
                )))
                .with_reset(Reset::WithPlacement(best));
        }

        MiddlewareReturn::empty().with_data(MiddlewareDataEntry::AutoPlacement(
            AutoPlacementData::new(next_index, overflows),
        ))
    }
}
