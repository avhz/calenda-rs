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

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPORTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::calendar::Calendar;
use crate::utilities::unpack_date;
use time::{Date, Month, Weekday};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Argentina national holiday calendar.
pub struct ArgentinaCalendar;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, METHODS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Calendar for ArgentinaCalendar {
    fn name(&self) -> &'static str {
        "Argentina"
    }

    fn is_holiday(&self, date: Date) -> bool {
        let (_, m, d, wd, yd, em) = unpack_date(date, false);

        if
        // New Year's Day
        (d == 1 && m == Month::January)
            // Holy Thursday
            || (yd == em-4)
            // Good Friday
            || (yd == em-3)
            // Labour Day
            || (d == 1 && m == Month::May)
            // May Revolution
            || (d == 25 && m == Month::May)
            // Death of General Manuel Belgrano
            || (d >= 15 && d <= 21 && wd == Weekday::Monday && m == Month::June)
            // Independence Day
            || (d == 9 && m == Month::July)
            // Death of General José de San Martín
            || (d >= 15 && d <= 21 && wd == Weekday::Monday && m == Month::August)
            // Columbus Day
            || ((d == 10 || d == 11 || d == 12 || d == 15 || d == 16) && wd == Weekday::Monday && m == Month::October)
            // Immaculate Conception
            || (d == 8 && m == Month::December)
            // Christmas Eve
            || (d == 24 && m == Month::December)
            // New Year's Eve
            || ((d == 31 || (d == 30 && wd == Weekday::Friday)) && m == Month::December)
        {
            return true;
        }

        false
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
