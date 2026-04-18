//! Library entry point.
//!
//! This crate exposes two main modules:
//! - `calendar`: internal logic for computing liturgical dates
//! - `ffi`: C-compatible API for external languages (Swift, Kotlin, C++, Python, etc.)

pub mod calendar;
pub mod ffi;
