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

    /// Function to list all holidays for a given range of `Date`s.
    fn all_holidays_between(&self, start_date: Date, end_date: Date) -> Vec<Date> {
        let mut holidays = Vec::new();

        let mut temp_date = start_date;

        while temp_date <= end_date {
            if self.is_holiday(temp_date) {
                holidays.push(temp_date);
            }

            temp_date = temp_date.next_day().unwrap();
        }

        holidays.sort();

        holidays
    }

    /// Function to list all business days for a given range of `Date`s.
    fn all_business_days_between(&self, start_date: Date, end_date: Date) -> Vec<Date> {
        let mut business_days = Vec::new();

        let mut temp_date = start_date;

        while temp_date <= end_date {
            if self.is_business_day(temp_date) {
                business_days.push(temp_date);
            }

            temp_date = temp_date.next_day().unwrap();
        }

        business_days.sort();

        business_days
    }
}
