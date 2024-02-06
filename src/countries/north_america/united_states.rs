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

/// United States of America national holiday calendar.
pub struct UnitedStatesCalendar;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// IMPLEMENTATIONS, METHODS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl Calendar for UnitedStatesCalendar {
    fn name(&self) -> &'static str {
        "United States of America"
    }

    fn is_holiday(&self, date: Date) -> bool {
        let (y, m, d, wd, _, _) = unpack_date(date, false);

        if (
            // New Year's Day (possibly moved to Monday if on Sunday)
            ((d == 1 || (d == 2 && wd == Weekday::Monday)) && m == Month::January)

            // (or to Friday if on Saturday)
            || (d == 31 && wd == Weekday::Friday && m == Month::December)

            // Martin Luther King's birthday (third Monday in January)
            || ((d >= 15 && d <= 21) && wd == Weekday::Monday && m == Month::January && y >= 1983)

            // Washington's birthday (third Monday in February)
            || self.is_washington_birthday(date)

            // Memorial Day (last Monday in May)
            || self.is_memorial_day(date)

            // Juneteenth (Monday if Sunday or Friday if Saturday)
            || self.is_juneteenth(date, true)

            // Independence Day (Monday if Sunday or Friday if Saturday)
            || ((d == 4 || (d == 5 && wd == Weekday::Monday) || (d == 3 && wd == Weekday::Friday)) && m == Month::July)

            // Labor Day (first Monday in September)
            || self.is_labor_day(date)

            // Columbus Day (second Monday in October)
            || self.is_columbus_day(date)

            // Veteran's Day (Monday if Sunday or Friday if Saturday)
            || self.is_veterans_day(date)

            // Thanksgiving Day (fourth Thursday in November)
            || ((d >= 22 && d <= 28) && wd == Weekday::Thursday && m == Month::November)

            // Christmas (Monday if Sunday or Friday if Saturday)
            || ((d == 25 || (d == 26 && wd == Weekday::Monday) || (d == 24 && wd == Weekday::Friday)) && m == Month::December)
        ) {
            return true;
        }

        false
    }
}

impl UnitedStatesCalendar {
    /// Create a new instance of the United States of America calendar.
    pub fn new() -> Self {
        UnitedStatesCalendar
    }

    fn is_washington_birthday(&self, date: Date) -> bool {
        let (y, m, d, wd, _, _) = unpack_date(date, false);

        if (y >= 1971) {
            // third Monday in February
            return (d >= 15 && d <= 21) && wd == Weekday::Monday && m == Month::February;
        } else {
            // February 22nd, possibly adjusted
            return (d == 22
                || (d == 23 && wd == Weekday::Monday)
                || (d == 21 && wd == Weekday::Friday))
                && m == Month::February;
        }
    }

    fn is_memorial_day(&self, date: Date) -> bool {
        let (y, m, d, wd, _, _) = unpack_date(date, false);

        if (y >= 1971) {
            // last Monday in May
            return d >= 25 && wd == Weekday::Monday && m == Month::May;
        } else {
            // May 30th, possibly adjusted
            return (d == 30
                || (d == 31 && wd == Weekday::Monday)
                || (d == 29 && wd == Weekday::Friday))
                && m == Month::May;
        }
    }

    fn is_labor_day(&self, date: Date) -> bool {
        let (_, m, d, wd, _, _) = unpack_date(date, false);

        // first Monday in September
        return d <= 7 && wd == Weekday::Monday && m == Month::September;
    }

    fn is_columbus_day(&self, date: Date) -> bool {
        let (y, m, d, wd, _, _) = unpack_date(date, false);

        // second Monday in October
        return (d >= 8 && d <= 14) && wd == Weekday::Monday && m == Month::October && y >= 1971;
    }

    fn is_veterans_day(&self, date: Date) -> bool {
        let (y, m, d, wd, _, _) = unpack_date(date, false);

        if (y <= 1970 || y >= 1978) {
            // November 11th, adjusted
            return (d == 11
                || (d == 12 && wd == Weekday::Monday)
                || (d == 10 && wd == Weekday::Friday))
                && m == Month::November;
        } else {
            // fourth Monday in October
            return (d >= 22 && d <= 28) && wd == Weekday::Monday && m == Month::October;
        }
    }

    fn is_juneteenth(&self, date: Date, move_to_friday: bool) -> bool {
        let (y, m, d, wd, _, _) = unpack_date(date, false);

        // declared in 2021, but only observed by exchanges since 2022
        return (d == 19
            || (d == 20 && wd == Weekday::Monday)
            || ((d == 18 && wd == Weekday::Friday) && move_to_friday))
            && m == Month::June
            && y >= 2022;
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
