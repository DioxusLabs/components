//! DOM platform implementation for the floating-ui positioning engine.
//!
//! This crate provides [`DomPlatform`], which implements the
//! [`Platform`](floating_ui_core::platform::Platform) trait using `web-sys`
//! for DOM measurement, as well as [`auto_update`] for watching layout changes.

pub mod auto_update;
pub mod platform;
pub mod utils;
