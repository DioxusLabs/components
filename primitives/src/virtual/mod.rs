//! Framework-agnostic virtual list implementation.
//!
//! This module provides the core algorithms needed for efficient list virtualization:
//!
//! - Computing item positions from measured or estimated sizes
//! - Calculating the visible range using binary search
//! - Handling scroll position corrections when items resize

mod types;
mod utils;
mod virtualizer;

pub(crate) use types::Rect;

pub(crate) use virtualizer::{Virtualizer, VirtualizerOptions};
