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

//! This module defines calendars and holidays for different countries.

/// Calendars implemented for African countries.
pub mod africa;

/// Calendars implemented for Asian countries.
pub mod asia;

/// Calendars implemented for European countries.
pub mod europe;

/// Calendars implemented for North American countries.
pub mod north_america;

/// Calendars implemented for Oceanian countries.
pub mod oceania {
    pub mod australia;
}

/// Calendars implemented for South American countries.
pub mod south_america;
