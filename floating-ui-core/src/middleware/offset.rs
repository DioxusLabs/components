//! The offset middleware — creates distance between the reference and floating elements.

use crate::middleware::{
    Middleware, MiddlewareDataEntry, MiddlewareReturn, MiddlewareState, OffsetData,
};
use crate::types::{Alignment, Side};

/// Creates a gap between the reference and floating elements.
///
/// `main_axis` controls the distance along the placement side (e.g., the vertical
/// gap for a `Bottom` placement). `cross_axis` shifts perpendicular to that.
/// `alignment_axis` replaces the cross-axis shift specifically for aligned placements.
pub struct Offset {
    main_axis: f64,
    cross_axis: f64,
    alignment_axis: Option<f64>,
}

impl Offset {
    /// Create with the given main-axis offset. Cross and alignment axes default to 0.
    pub fn new(main_axis: f64) -> Self {
        Self {
            main_axis,
            cross_axis: 0.0,
            alignment_axis: None,
        }
    }

    /// Set the cross-axis offset.
    pub fn cross_axis(mut self, v: f64) -> Self {
        self.cross_axis = v;
        self
    }

    /// Set the alignment-axis offset (only applies to aligned placements like `TopStart`).
    pub fn alignment_axis(mut self, v: f64) -> Self {
        self.alignment_axis = Some(v);
        self
    }
}

impl<E> Middleware<E> for Offset {
    fn name(&self) -> &'static str {
        "offset"
    }

    fn compute(&self, state: &MiddlewareState<E>) -> MiddlewareReturn {
        let placement = state.placement();
        let side = placement.side();
        let alignment = placement.alignment();

        let main_axis = self.main_axis;
        let mut cross_axis = self.cross_axis;

        // If alignment_axis is set and this is an aligned placement, use it
        // instead of cross_axis. For 'end' alignment, invert.
        if let Some(alignment_val) = self.alignment_axis {
            if alignment.is_some() {
                cross_axis = match alignment {
                    Some(Alignment::End) => -alignment_val,
                    _ => alignment_val,
                };
            }
        }

        // Convert logical axes to physical x/y
        // For "origin" sides (top/left), main axis is inverted
        let main_axis_multi = match side {
            Side::Top | Side::Left => -1.0,
            Side::Bottom | Side::Right => 1.0,
        };

        let is_vertical = side.is_vertical();
        let rtl = state.platform().is_rtl(state.elements().floating());
        let cross_axis_multi = if rtl && is_vertical { -1.0 } else { 1.0 };

        let (diff_x, diff_y) = if is_vertical {
            // Vertical placement (top/bottom): main is y, cross is x
            (cross_axis * cross_axis_multi, main_axis * main_axis_multi)
        } else {
            // Horizontal placement (left/right): main is x, cross is y
            (main_axis * main_axis_multi, cross_axis * cross_axis_multi)
        };

        MiddlewareReturn::empty()
            .with_coords(state.x() + diff_x, state.y() + diff_y)
            .with_data(MiddlewareDataEntry::Offset(OffsetData::new(
                diff_x, diff_y, placement,
            )))
    }
}
