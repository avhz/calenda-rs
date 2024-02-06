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

use crate::functions::is_weekend;
use time::Date;

/// Calendar trait.
pub trait Calendar {
    /// Name of the calendar, typically the country name, but could also be
    /// a region/subdivision or a special calendar, such as a financial calendar (e.g. NYSE).
    fn name(&self) -> &'static str;

    /// Check if the date is a holiday (but not a weekend).
    /// This is the primary method to implement for a calendar.
    fn is_holiday(&self, date: Date) -> bool;

    /// Check if the date is a business day.
    /// A business day is a day that is not a holiday and not a weekend.
    fn is_business_day(&self, date: Date) -> bool {
        !is_weekend(date) && !self.is_holiday(date)
    }

    // /// List of all holidays for the given calendar and year.
    // fn holidays(&self, year: i32) -> BTreeMap<Date, Holiday> {
    //     todo!("Implement holidays")
    // }
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
