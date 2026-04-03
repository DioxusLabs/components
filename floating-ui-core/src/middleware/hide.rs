//! The hide middleware — detects when the reference or floating element
//! should be hidden due to clipping.

use crate::detect_overflow::{detect_overflow, DetectOverflowOptions};
use crate::middleware::{
    HideData, Middleware, MiddlewareDataEntry, MiddlewareReturn, MiddlewareState,
};
use crate::types::ElementContext;

/// Which hiding condition to check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HideStrategy {
    /// The reference element is fully clipped by an ancestor.
    ReferenceHidden,
    /// The floating element has escaped the reference's clipping boundary.
    Escaped,
}

/// Detects when the reference or floating element should be visually hidden
/// because it has been clipped or has escaped its boundary.
///
/// This middleware only produces data — it does not modify coordinates.
/// Use the returned data to set `visibility: hidden` on the floating element.
pub struct Hide<E: 'static> {
    strategy: HideStrategy,
    detect_overflow_options: DetectOverflowOptions<E>,
}

impl<E> Hide<E> {
    /// Create with the given strategy.
    pub fn new(strategy: HideStrategy) -> Self {
        Self {
            strategy,
            detect_overflow_options: DetectOverflowOptions::new(),
        }
    }

    /// Create a "reference hidden" checker.
    pub fn reference_hidden() -> Self {
        Self::new(HideStrategy::ReferenceHidden)
    }

    /// Create an "escaped" checker.
    pub fn escaped() -> Self {
        Self::new(HideStrategy::Escaped)
    }

    /// Set overflow detection options.
    pub fn detect_overflow(mut self, opts: DetectOverflowOptions<E>) -> Self {
        self.detect_overflow_options = opts;
        self
    }
}

impl<E> Middleware<E> for Hide<E> {
    fn name(&self) -> &'static str {
        "hide"
    }

    fn compute(&self, state: &MiddlewareState<E>) -> MiddlewareReturn {
        match self.strategy {
            HideStrategy::ReferenceHidden => {
                // Check if the reference element is fully clipped
                let mut opts =
                    DetectOverflowOptions::new().element_context(ElementContext::Reference);

                // Copy padding from user options
                opts = opts.padding(*self.detect_overflow_options.get_padding());

                let overflow = detect_overflow(state, &opts);

                // The reference is hidden if all sides have positive overflow
                // (meaning the reference is completely outside the clipping boundary)
                let reference_hidden = overflow.top >= 0.0
                    && overflow.bottom >= 0.0
                    && overflow.left >= 0.0
                    && overflow.right >= 0.0;

                let existing = state.middleware_data().hide();
                let escaped = existing.map(|d| d.escaped()).unwrap_or(false);
                let escaped_offsets = existing.and_then(|d| d.escaped_offsets().copied());

                MiddlewareReturn::empty().with_data(MiddlewareDataEntry::Hide(HideData::new(
                    reference_hidden,
                    escaped,
                    Some(overflow),
                    escaped_offsets,
                )))
            }
            HideStrategy::Escaped => {
                // Check if the floating element has escaped the reference's boundary
                let mut opts = DetectOverflowOptions::new().alt_boundary(true);

                opts = opts.padding(*self.detect_overflow_options.get_padding());

                let overflow = detect_overflow(state, &opts);

                // The floating element has escaped if any side has positive overflow
                // relative to the reference's clipping boundary
                let escaped = overflow.top > 0.0
                    || overflow.bottom > 0.0
                    || overflow.left > 0.0
                    || overflow.right > 0.0;

                let existing = state.middleware_data().hide();
                let reference_hidden = existing.map(|d| d.reference_hidden()).unwrap_or(false);
                let reference_hidden_offsets =
                    existing.and_then(|d| d.reference_hidden_offsets().copied());

                MiddlewareReturn::empty().with_data(MiddlewareDataEntry::Hide(HideData::new(
                    reference_hidden,
                    escaped,
                    reference_hidden_offsets,
                    Some(overflow),
                )))
            }
        }
    }
}
