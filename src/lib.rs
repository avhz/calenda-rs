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

#![forbid(missing_docs)]

/// The core `Calendar` trait.
pub mod calendar;

/// Constants for calendars and holidays.
#[allow(dead_code)]
pub mod constants;

/// Calendars implemented for specific countries.
pub mod countries;

/// Error types and functions.
pub mod errors;

/// Functions for working with dates and times.
pub mod functions;

/// `Holiday` trait.
pub mod holiday;

// /// Macro to create a `Holiday` constant.
// #[macro_export]
// macro_rules! holiday {
//     ($name:literal, $date:expr, $description:literal) => {
//         $crate::holiday::Holiday {
//             name: $name,
//             date: $date,
//             description: Some($description),
//         }
//     };
// }

// macro_rules! generate_holiday_constants_for_country_and_year_range {
//     ($country_code:ident, $start_year:literal, $end_year:literal, $($name:literal, $date:literal, $description:literal,)*) => {
//         $(
//             /// $name for year $current_year
//             pub const $country_code: Holiday = Holiday {
//                 name: $name,
//                 date: $date,
//                 description: Some($description),
//             };
//         )*
//     };
// }

// generate_holiday_constants_for_country_and_year_range! {
//     // Australia
//     AU, 2024, 2024,
//     "New Year's Day", date!(2024-01-01), "New Year's Day",
//     "Australia Day", date!(2024-01-26), "Australia Day",
//     "Good Friday", date!(2024-04-05), "Good Friday",
//     "Easter Monday", date!(2024-04-08), "Easter Monday",
//     "ANZAC Day", date!(2024-04-25), "ANZAC Day",
//     "Queen's Birthday", date!(2024-06-10), "Queen's Birthday",
//     "Bank Holiday", date!(2024-08-05), "Bank Holiday",
//     "Labour Day", date!(2024-10-07), "Labour Day",
//     "Christmas Day", date!(2024-12-25), "Christmas Day",
//     "Boxing Day", date!(2024-12-26), "Boxing Day",
//     "National Day of Mourning for Her Majesty", date!(2024-09-22), "National Day of Mourning for Her Majesty",
// }
