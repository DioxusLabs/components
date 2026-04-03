//! The inline middleware — handles multi-line reference elements.
//!
//! When a reference element spans multiple lines (e.g., a hyperlink),
//! its bounding rect covers the entire area. This middleware uses
//! individual client rects to select the most relevant line for positioning.

use euclid::{Point2D, Size2D};

use crate::middleware::{
    InlineData, Middleware, MiddlewareDataEntry, MiddlewareReturn, MiddlewareState, Reset,
};
use crate::types::{ClientRect, ElementRects, LayoutRect, Side};

/// Handles positioning relative to multi-line inline reference elements.
///
/// Uses individual client rects (one per line) instead of the overall
/// bounding box, selecting the most relevant line rect based on the
/// placement and optional coordinates.
pub struct Inline {
    x: Option<f64>,
    y: Option<f64>,
    padding: f64,
}

impl Inline {
    /// Create with defaults.
    pub fn new() -> Self {
        Self {
            x: None,
            y: None,
            padding: 2.0,
        }
    }

    /// Set coordinates to help select which line to anchor to
    /// (e.g., the cursor position for a context menu).
    pub fn coords(mut self, x: f64, y: f64) -> Self {
        self.x = Some(x);
        self.y = Some(y);
        self
    }

    /// Set padding for rect matching tolerance.
    pub fn padding(mut self, p: f64) -> Self {
        self.padding = p;
        self
    }
}

impl Default for Inline {
    fn default() -> Self {
        Self::new()
    }
}

/// Group client rects by line (rects within the same vertical band).
fn get_rects_by_line(rects: &[ClientRect]) -> Vec<LayoutRect> {
    if rects.is_empty() {
        return vec![];
    }

    let mut lines: Vec<Vec<&ClientRect>> = Vec::new();

    for rect in rects {
        let mut found = false;
        for line in &mut lines {
            // If this rect is within half its height of the line's first rect,
            // consider it on the same line
            let line_top = line[0].top();
            let threshold = rect.height() / 2.0;
            if (rect.top() - line_top).abs() < threshold {
                line.push(rect);
                found = true;
                break;
            }
        }
        if !found {
            lines.push(vec![rect]);
        }
    }

    // For each line, compute the bounding rect
    lines
        .into_iter()
        .map(|line| {
            let min_x = line.iter().map(|r| r.left()).fold(f64::INFINITY, f64::min);
            let max_x = line
                .iter()
                .map(|r| r.right())
                .fold(f64::NEG_INFINITY, f64::max);
            let min_y = line.iter().map(|r| r.top()).fold(f64::INFINITY, f64::min);
            let max_y = line
                .iter()
                .map(|r| r.bottom())
                .fold(f64::NEG_INFINITY, f64::max);

            LayoutRect::new(
                Point2D::new(min_x, min_y),
                Size2D::new(max_x - min_x, max_y - min_y),
            )
        })
        .collect()
}

impl<E> Middleware<E> for Inline {
    fn name(&self) -> &'static str {
        "inline"
    }

    fn compute(&self, state: &MiddlewareState<E>) -> MiddlewareReturn {
        let placement = state.placement();
        let side = placement.side();
        let client_rects = state
            .platform()
            .get_client_rects(state.elements().reference());

        if client_rects.is_empty() {
            return MiddlewareReturn::empty()
                .with_data(MiddlewareDataEntry::Inline(InlineData::new()));
        }

        let line_rects = get_rects_by_line(&client_rects);

        if line_rects.is_empty() {
            return MiddlewareReturn::empty()
                .with_data(MiddlewareDataEntry::Inline(InlineData::new()));
        }

        // Select the most relevant line rect
        let selected = if let (Some(x), Some(y)) = (self.x, self.y) {
            // Find the rect that contains the given coordinates
            let padding = self.padding;
            line_rects
                .iter()
                .find(|r| {
                    x >= r.min_x() - padding
                        && x <= r.max_x() + padding
                        && y >= r.min_y() - padding
                        && y <= r.max_y() + padding
                })
                .copied()
                .unwrap_or_else(|| {
                    // Fallback: pick based on placement
                    select_by_placement(&line_rects, side)
                })
        } else {
            select_by_placement(&line_rects, side)
        };

        // Check if the selected rect differs significantly from the current reference rect
        let current_ref = state.rects().reference();
        let diff = (selected.origin.x - current_ref.origin.x).abs()
            + (selected.origin.y - current_ref.origin.y).abs()
            + (selected.size.width - current_ref.size.width).abs()
            + (selected.size.height - current_ref.size.height).abs();

        if diff > 1.0 {
            // Reset with the selected line rect as the reference
            let new_rects = ElementRects::new(selected, state.rects().floating());
            return MiddlewareReturn::empty()
                .with_data(MiddlewareDataEntry::Inline(InlineData::new()))
                .with_reset(Reset::WithRects(new_rects));
        }

        MiddlewareReturn::empty().with_data(MiddlewareDataEntry::Inline(InlineData::new()))
    }
}

/// Select a line rect based on the placement side.
fn select_by_placement(line_rects: &[LayoutRect], side: Side) -> LayoutRect {
    match side {
        Side::Top => {
            // Use the first (topmost) line
            *line_rects.first().unwrap()
        }
        Side::Bottom => {
            // Use the last (bottommost) line
            *line_rects.last().unwrap()
        }
        Side::Left => {
            // Use the leftmost line
            *line_rects
                .iter()
                .min_by(|a, b| a.min_x().partial_cmp(&b.min_x()).unwrap())
                .unwrap()
        }
        Side::Right => {
            // Use the rightmost line
            *line_rects
                .iter()
                .max_by(|a, b| a.max_x().partial_cmp(&b.max_x()).unwrap())
                .unwrap()
        }
    }
}
