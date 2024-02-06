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

//! This module defines a `Calendar` type and its methods.

use crate::{functions::is_weekend, holiday::Holiday};
use std::collections::BTreeMap;
use time::Date;

/// Calendar trait.
pub trait Calendar {
    /// Name of the calendar.
    fn name(&self) -> &'static str;

    /// Check if the date is a business day.
    fn is_business_day(&self, date: Date) -> bool {
        let is_weekend = is_weekend(date);
        let is_holiday = self.is_holiday(date);

        !is_weekend && !is_holiday
    }

    /// Check if the date is a holiday.
    fn is_holiday(&self, date: Date) -> bool {
        !is_weekend(date) && self.is_business_day(date)
    }

    /// List of all holidays for the given calendar and year.
    fn holidays(&self, year: i32) -> BTreeMap<Date, Holiday>;
    //     let mut holidays = BTreeMap::new();

    //     for day in 1..=days_in_year(year) {
    //         let date = Date::from_ordinal_date(year, day);

    //         if self.is_holiday(date.unwrap_or(false)) {
    //             holidays.insert(date, Holiday::default());
    //         }
    //     }

    //     holidays
    // }

    // /// Returns the ISO 3166-1 country code.
    // fn country_code(&self) -> crate::iso::ISO_3166;
}
