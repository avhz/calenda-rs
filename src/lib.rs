// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// calenda-rs: A Rust library for global calendars.
// Copyright (C) 2024 https://github.com/avhz
//
// Dual licensed under Apache 2.0 and MIT.
//
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! `calenda-rs` is a Rust library for creating and working with global calendars.
//! This library is a [RustQuant project](github.com/avhz/RustQuant).

#![forbid(missing_docs)]

/// The core `Calendar` trait.
pub mod calendar;

/// Constants for calendars and holidays.
#[allow(dead_code)]
pub mod constants;

/// Conventions for day counting.
pub mod conventions;

/// Calendars implemented for specific countries.
#[allow(unused_parens)]
pub mod countries;

// /// Day counting.
// pub mod daycount;

/// Functions for working with dates and times.
pub mod functions;

/// The `Holiday` trait.
pub mod holiday;
