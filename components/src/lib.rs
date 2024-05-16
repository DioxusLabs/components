#![allow(non_snake_case)]

pub mod input;
pub mod layout;
pub mod style;

// Spacing interval of 8px (4..8..12)
pub(crate) const SPACING_INTERVAL: u8 = 4;

// TODO: Replace commonly duplicated code with macro (e.g. provide context, subscribe to prop)