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
use crate::functions::unpack_date;
use time::{Date, Month, Weekday};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// STRUCTS, ENUMS, TRAITS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Canada national holiday calendar.
pub struct CanadaCalendar;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, METHODS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Calendar for CanadaCalendar {
    fn name(&self) -> &'static str {
        "Canada"
    }

    fn is_holiday(&self, date: Date) -> bool {
        let (y, m, d, wd, yd, em) = unpack_date(date, false);

        if (
            // New Year's Day (possibly moved to Monday)
            ((d == 1 || ((d == 2 || d == 3) && wd == Weekday::Monday)) && m == Month::January)

            // Family Day (third Monday in February, since 2008)
            || ((d >= 15 && d <= 21) && wd == Weekday::Monday && m == Month::February && y >= 2008)

            // Good Friday
            || (yd == em-3)

            // The Monday on or preceding 24 May (Victoria Day)
            || (d > 17 && d <= 24 && wd == Weekday::Monday && m == Month::May)

            // July 1st, possibly moved to Monday (Canada Day)
            || ((d == 1 || ((d == 2 || d == 3) && wd == Weekday::Monday)) && m==Month::July)

            // first Monday of August (Provincial Holiday)
            || (d <= 7 && wd == Weekday::Monday && m == Month::August)

            // first Monday of September (Labor Day)
            || (d <= 7 && wd == Weekday::Monday && m == Month::September)

            // September 30th, possibly moved to Monday
            // (National Day for Truth and Reconciliation, since 2021)
            || (((d == 30 && m == Month::September) || (d <= 2 && m == Month::October && wd == Weekday::Monday)) && y >= 2021)

            // second Monday of October (Thanksgiving Day)
            || (d > 7 && d <= 14 && wd == Weekday::Monday && m == Month::October)

            // November 11th (possibly moved to Monday)
            || ((d == 11 || ((d == 12 || d == 13) && wd == Weekday::Monday)) && m == Month::November)

            // Christmas (possibly moved to Monday or Tuesday)
            || ((d == 25 || (d == 27 && (wd == Weekday::Monday || wd == Weekday::Tuesday))) && m == Month::December)

            // Boxing Day (possibly moved to Monday or Tuesday)
            || ((d == 26 || (d == 28 && (wd == Weekday::Monday || wd == Weekday::Tuesday))) && m == Month::December)
        ) {
            return true;
        }

        false
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
