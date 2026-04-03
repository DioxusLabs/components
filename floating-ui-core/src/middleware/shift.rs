//! The shift middleware — slides the floating element to prevent overflow.

use crate::detect_overflow::{detect_overflow, DetectOverflowOptions};
use crate::middleware::{
    Middleware, MiddlewareDataEntry, MiddlewareReturn, MiddlewareState, ShiftData,
};
use crate::types::Point;

/// A limiter constrains how far the shift middleware can move the floating element.
pub trait Limiter<E> {
    /// Given the proposed shift offset, return the limited offset.
    fn compute(&self, state: &MiddlewareState<E>, offset: Point) -> Point;
}

/// Prevents the floating element from shifting so far that it detaches
/// from its reference element.
pub struct LimitShift {
    offset: f64,
    main_axis: bool,
    cross_axis: bool,
}

impl LimitShift {
    /// Create with default settings (offset 0, limit both axes).
    pub fn new() -> Self {
        Self {
            offset: 0.0,
            main_axis: true,
            cross_axis: true,
        }
    }

    /// Set the maximum distance the floating element can shift past the
    /// reference element's edge.
    pub fn offset(mut self, v: f64) -> Self {
        self.offset = v;
        self
    }

    /// Whether to limit on the main axis.
    pub fn main_axis(mut self, v: bool) -> Self {
        self.main_axis = v;
        self
    }

    /// Whether to limit on the cross axis.
    pub fn cross_axis(mut self, v: bool) -> Self {
        self.cross_axis = v;
        self
    }
}

impl Default for LimitShift {
    fn default() -> Self {
        Self::new()
    }
}

impl<E> Limiter<E> for LimitShift {
    fn compute(&self, state: &MiddlewareState<E>, offset: Point) -> Point {
        let placement = state.placement();
        let side = placement.side();
        let is_vertical = side.is_vertical();

        let rects = state.rects();
        let reference = rects.reference();
        let floating = rects.floating();

        let main_axis_coord = if is_vertical { offset.y } else { offset.x };
        let cross_axis_coord = if is_vertical { offset.x } else { offset.y };

        let limited_main = if self.main_axis {
            let (ref_start, ref_len, float_len) = if is_vertical {
                (
                    reference.origin.y,
                    reference.size.height,
                    floating.size.height,
                )
            } else {
                (
                    reference.origin.x,
                    reference.size.width,
                    floating.size.width,
                )
            };

            let min = ref_start - float_len + self.offset;
            let max = ref_start + ref_len - self.offset;
            main_axis_coord.clamp(min, max)
        } else {
            main_axis_coord
        };

        let limited_cross = if self.cross_axis {
            let (ref_start, ref_len, float_len) = if is_vertical {
                (
                    reference.origin.x,
                    reference.size.width,
                    floating.size.width,
                )
            } else {
                (
                    reference.origin.y,
                    reference.size.height,
                    floating.size.height,
                )
            };

            let min = ref_start - float_len + self.offset;
            let max = ref_start + ref_len - self.offset;
            cross_axis_coord.clamp(min, max)
        } else {
            cross_axis_coord
        };

        if is_vertical {
            euclid::Point2D::new(limited_cross, limited_main)
        } else {
            euclid::Point2D::new(limited_main, limited_cross)
        }
    }
}

/// Shifts the floating element along its axes to prevent overflow without
/// changing the placement.
pub struct Shift<E: 'static> {
    main_axis: bool,
    cross_axis: bool,
    limiter: Option<Box<dyn Limiter<E>>>,
    detect_overflow_options: DetectOverflowOptions<E>,
}

impl<E> Shift<E> {
    /// Create with defaults: shift on main axis only, no limiter.
    pub fn new() -> Self {
        Self {
            main_axis: true,
            cross_axis: false,
            limiter: None,
            detect_overflow_options: DetectOverflowOptions::new(),
        }
    }

    /// Whether to shift on the main axis.
    pub fn main_axis(mut self, v: bool) -> Self {
        self.main_axis = v;
        self
    }

    /// Whether to shift on the cross axis.
    pub fn cross_axis(mut self, v: bool) -> Self {
        self.cross_axis = v;
        self
    }

    /// Set a limiter to constrain shifting distance.
    pub fn limiter(mut self, l: Box<dyn Limiter<E>>) -> Self {
        self.limiter = Some(l);
        self
    }

    /// Set overflow detection options.
    pub fn detect_overflow(mut self, opts: DetectOverflowOptions<E>) -> Self {
        self.detect_overflow_options = opts;
        self
    }
}

impl<E> Default for Shift<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E> Middleware<E> for Shift<E> {
    fn name(&self) -> &'static str {
        "shift"
    }

    fn compute(&self, state: &MiddlewareState<E>) -> MiddlewareReturn {
        let placement = state.placement();
        let side = placement.side();
        let is_vertical = side.is_vertical();

        let overflow = detect_overflow(state, &self.detect_overflow_options);

        // Determine axis pairs based on placement orientation
        let (main_overflow_start, main_overflow_end) = if is_vertical {
            // For top/bottom: main axis is x (left/right overflow)
            // Wait — actually for shift, the main axis is the *alignment* axis,
            // which for top/bottom is x. For left/right is y.
            (overflow.left, overflow.right)
        } else {
            (overflow.top, overflow.bottom)
        };

        let (cross_overflow_start, cross_overflow_end) = if is_vertical {
            (overflow.top, overflow.bottom)
        } else {
            (overflow.left, overflow.right)
        };

        let mut x = state.x();
        let mut y = state.y();

        // Shift on main axis (the alignment axis — perpendicular to the side)
        if self.main_axis {
            let coord = if is_vertical { &mut x } else { &mut y };

            // Shift toward start if overflowing end, toward end if overflowing start
            if main_overflow_start > 0.0 {
                *coord += main_overflow_start;
            }
            if main_overflow_end > 0.0 {
                *coord -= main_overflow_end;
            }
        }

        // Shift on cross axis (the side axis)
        if self.cross_axis {
            let coord = if is_vertical { &mut y } else { &mut x };
            if cross_overflow_start > 0.0 {
                *coord += cross_overflow_start;
            }
            if cross_overflow_end > 0.0 {
                *coord -= cross_overflow_end;
            }
        }

        // Apply limiter
        let (x, y) = if let Some(ref limiter) = self.limiter {
            let limited = limiter.compute(state, euclid::Point2D::new(x, y));
            (limited.x, limited.y)
        } else {
            (x, y)
        };

        let shift_x = x - state.x();
        let shift_y = y - state.y();

        MiddlewareReturn::empty()
            .with_coords(x, y)
            .with_data(MiddlewareDataEntry::Shift(ShiftData::new(shift_x, shift_y)))
    }
}
