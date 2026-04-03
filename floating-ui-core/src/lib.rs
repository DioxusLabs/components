//! Platform-agnostic floating UI positioning engine.
//!
//! This crate provides the core positioning algorithm and middleware pipeline
//! for computing the position of floating elements relative to reference elements.
//! It is inspired by the [floating-ui](https://floating-ui.com/) library.
//!
//! The crate is platform-agnostic — all DOM/environment interactions are abstracted
//! behind the [`Platform`] trait. See `floating-ui-dom` for a web-sys implementation.

pub mod compute_coords;
pub mod compute_position;
pub mod detect_overflow;
pub mod middleware;
pub mod platform;
pub mod premeasured;
pub mod types;
