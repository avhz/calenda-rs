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

//! This module defines general calendar and holiday related functions.

use crate::constants::EASTER_MONDAYS;
use time::{util::days_in_year_month, Date, Duration, Error, Month, Weekday};

/// Unpacks a `Date` into a tuple in the following form:
///
/// ```ignore
/// (
///     y,      // Year
///     m,      // Month (January - December)
///     d,      // Day of month (1 - 31)
///     wd,     // Weekday (Monday-Sunday)
///     yd,     // Day of year (1 - 365)
///     em,     // Easter Monday
/// )
/// ```
pub fn unpack_date(date: Date, is_orthodox: bool) -> (i32, Month, u8, Weekday, u16, u16) {
    let y = date.year();
    let m = date.month();
    let d = date.day();

    let wd = date.weekday();
    let yd = date.ordinal();

    let em = easter_monday(y as usize, is_orthodox);

    (y, m, d, wd, yd, em)
}

/// Returns the Easter Monday for the given year.
fn easter_monday(year: usize, is_orthodox: bool) -> u16 {
    EASTER_MONDAYS[usize::from(is_orthodox)][year - 1901]
}

/// Checks if date is a weekend.
pub fn is_weekend(date: Date) -> bool {
    let w = date.weekday();

    w == time::Weekday::Saturday || w == time::Weekday::Sunday
}

/// Check if the date is a weekday.
pub fn is_weekday(date: Date) -> bool {
    !is_weekend(date)
}

/// Function to get the first day of the month.
pub fn get_first_day_of_month(year: i32, month: Month) -> Result<Weekday, Error> {
    Ok(Date::from_calendar_date(year, month, 1)?.weekday())
}

/// Function to get the last day of the month.
pub fn get_last_day_of_month(year: i32, month: Month) -> Result<Weekday, Error> {
    let date = Date::from_calendar_date(year, month, 1)?;

    let last_day = date + Duration::days(days_in_year_month(year, month) as i64);

    Ok(last_day.weekday())
}

/// Function to get the date of the first Monday of the month.
pub fn get_first_monday_of_month(year: i32, month: Month) -> Result<Date, Error> {
    let first_day_date = Date::from_calendar_date(year, month, 1)?;

    match first_day_date.weekday() {
        Weekday::Monday => Ok(first_day_date),
        _ => Ok(first_day_date.next_occurrence(Weekday::Monday)),
    }
}

/// Function to get the date of the first Tuesday of the month.
pub fn get_first_tuesday_of_month(year: i32, month: Month) -> Result<Date, Error> {
    let first_day_date = Date::from_calendar_date(year, month, 1)?;

    match first_day_date.weekday() {
        Weekday::Tuesday => Ok(first_day_date),
        _ => Ok(first_day_date.next_occurrence(Weekday::Tuesday)),
    }
}

/// Function to get the date of the first Wednesday of the month.
pub fn get_first_wednesday_of_month(year: i32, month: Month) -> Result<Date, Error> {
    let first_day_date = Date::from_calendar_date(year, month, 1)?;

    match first_day_date.weekday() {
        Weekday::Wednesday => Ok(first_day_date),
        _ => Ok(first_day_date.next_occurrence(Weekday::Wednesday)),
    }
}

/// Function to get the date of the first Thursday of the month.
pub fn get_first_thursday_of_month(year: i32, month: Month) -> Result<Date, Error> {
    let first_day_date = Date::from_calendar_date(year, month, 1)?;

    match first_day_date.weekday() {
        Weekday::Thursday => Ok(first_day_date),
        _ => Ok(first_day_date.next_occurrence(Weekday::Thursday)),
    }
}

/// Function to get the date of the first Friday of the month.
pub fn get_first_friday_of_month(year: i32, month: Month) -> Result<Date, Error> {
    let first_day_date = Date::from_calendar_date(year, month, 1)?;

    match first_day_date.weekday() {
        Weekday::Friday => Ok(first_day_date),
        _ => Ok(first_day_date.next_occurrence(Weekday::Friday)),
    }
}

/// Function to get the date of the first Saturday of the month.
pub fn get_first_saturday_of_month(year: i32, month: Month) -> Result<Date, Error> {
    let first_day_date = Date::from_calendar_date(year, month, 1)?;

    match first_day_date.weekday() {
        Weekday::Saturday => Ok(first_day_date),
        _ => Ok(first_day_date.next_occurrence(Weekday::Saturday)),
    }
}

/// Function to get the date of the first Sunday of the month.
pub fn get_first_sunday_of_month(year: i32, month: Month) -> Result<Date, Error> {
    let first_day_date = Date::from_calendar_date(year, month, 1)?;

    match first_day_date.weekday() {
        Weekday::Sunday => Ok(first_day_date),
        _ => Ok(first_day_date.next_occurrence(Weekday::Sunday)),
    }
}

/// Function to get the date of the last Monday of the month.
pub fn get_last_monday_of_month(year: i32, month: Month) -> Result<Date, Error> {
    let days_in_month = days_in_year_month(year, month);
    let last_day_date = Date::from_calendar_date(year, month, days_in_month)?;

    match last_day_date.weekday() {
        Weekday::Monday => Ok(last_day_date),
        _ => Ok(last_day_date.prev_occurrence(Weekday::Monday)),
    }
}

/// Function to get the date of the last Tuesday of the month.
pub fn get_last_tuesday_of_month(year: i32, month: Month) -> Result<Date, Error> {
    let days_in_month = days_in_year_month(year, month);
    let last_day_date = Date::from_calendar_date(year, month, days_in_month)?;

    match last_day_date.weekday() {
        Weekday::Tuesday => Ok(last_day_date),
        _ => Ok(last_day_date.prev_occurrence(Weekday::Tuesday)),
    }
}

/// Function to get the date of the last Wednesday of the month.
pub fn get_last_wednesday_of_month(year: i32, month: Month) -> Result<Date, Error> {
    let days_in_month = days_in_year_month(year, month);
    let last_day_date = Date::from_calendar_date(year, month, days_in_month)?;

    match last_day_date.weekday() {
        Weekday::Wednesday => Ok(last_day_date),
        _ => Ok(last_day_date.prev_occurrence(Weekday::Wednesday)),
    }
}

/// Function to get the date of the last Thursday of the month.
pub fn get_last_thursday_of_month(year: i32, month: Month) -> Result<Date, Error> {
    let days_in_month = days_in_year_month(year, month);
    let last_day_date = Date::from_calendar_date(year, month, days_in_month)?;

    match last_day_date.weekday() {
        Weekday::Thursday => Ok(last_day_date),
        _ => Ok(last_day_date.prev_occurrence(Weekday::Thursday)),
    }
}

/// Function to get the date of the last Friday of the month.
pub fn get_last_friday_of_month(year: i32, month: Month) -> Result<Date, Error> {
    let days_in_month = days_in_year_month(year, month);
    let last_day_date = Date::from_calendar_date(year, month, days_in_month)?;

    match last_day_date.weekday() {
        Weekday::Friday => Ok(last_day_date),
        _ => Ok(last_day_date.prev_occurrence(Weekday::Friday)),
    }
}

/// Function to get the date of the last Saturday of the month.
pub fn get_last_saturday_of_month(year: i32, month: Month) -> Result<Date, Error> {
    let days_in_month = days_in_year_month(year, month);
    let last_day_date = Date::from_calendar_date(year, month, days_in_month)?;

    match last_day_date.weekday() {
        Weekday::Saturday => Ok(last_day_date),
        _ => Ok(last_day_date.prev_occurrence(Weekday::Saturday)),
    }
}

/// Function to get the date of the last Sunday of the month.
pub fn get_last_sunday_of_month(year: i32, month: Month) -> Result<Date, Error> {
    let days_in_month = days_in_year_month(year, month);
    let last_day_date = Date::from_calendar_date(year, month, days_in_month)?;

    match last_day_date.weekday() {
        Weekday::Sunday => Ok(last_day_date),
        _ => Ok(last_day_date.prev_occurrence(Weekday::Sunday)),
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNIT TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod test_utilities {
    use super::*;

    #[test]
    fn test_first_x_day_of_month() {
        let y = 2024;
        let m = Month::January;

        assert_eq!(
            get_first_monday_of_month(y, m).unwrap(),
            Date::from_calendar_date(y, m, 1).unwrap()
        );
        assert_eq!(
            get_first_tuesday_of_month(y, m).unwrap(),
            Date::from_calendar_date(y, m, 2).unwrap()
        );
        assert_eq!(
            get_first_wednesday_of_month(y, m).unwrap(),
            Date::from_calendar_date(y, m, 3).unwrap()
        );
        assert_eq!(
            get_first_thursday_of_month(y, m).unwrap(),
            Date::from_calendar_date(y, m, 4).unwrap()
        );
        assert_eq!(
            get_first_friday_of_month(y, m).unwrap(),
            Date::from_calendar_date(y, m, 5).unwrap()
        );
        assert_eq!(
            get_first_saturday_of_month(y, m).unwrap(),
            Date::from_calendar_date(y, m, 6).unwrap()
        );
        assert_eq!(
            get_first_sunday_of_month(y, m).unwrap(),
            Date::from_calendar_date(y, m, 7).unwrap()
        );
    }

    #[test]
    fn test_last_x_day_of_month() {
        let y = 2024;
        let m = Month::January;

        assert_eq!(
            get_last_monday_of_month(y, m).unwrap(),
            Date::from_calendar_date(y, m, 29).unwrap()
        );
        assert_eq!(
            get_last_tuesday_of_month(y, m).unwrap(),
            Date::from_calendar_date(y, m, 30).unwrap()
        );
        assert_eq!(
            get_last_wednesday_of_month(y, m).unwrap(),
            Date::from_calendar_date(y, m, 31).unwrap()
        );
        assert_eq!(
            get_last_thursday_of_month(y, m).unwrap(),
            Date::from_calendar_date(y, m, 25).unwrap()
        );
        assert_eq!(
            get_last_friday_of_month(y, m).unwrap(),
            Date::from_calendar_date(y, m, 26).unwrap()
        );
        assert_eq!(
            get_last_saturday_of_month(y, m).unwrap(),
            Date::from_calendar_date(y, m, 27).unwrap()
        );
        assert_eq!(
            get_last_sunday_of_month(y, m).unwrap(),
            Date::from_calendar_date(y, m, 28).unwrap()
        );
    }

    #[test]
    fn test_get_first_day_of_month() {
        assert_eq!(
            get_first_day_of_month(2024, Month::January).unwrap(),
            Weekday::Monday
        );
        assert_eq!(
            get_first_day_of_month(2024, Month::February).unwrap(),
            Weekday::Thursday
        );
        assert_eq!(
            get_first_day_of_month(2024, Month::March).unwrap(),
            Weekday::Friday
        );
        assert_eq!(
            get_first_day_of_month(2024, Month::April).unwrap(),
            Weekday::Monday
        );
        assert_eq!(
            get_first_day_of_month(2024, Month::May).unwrap(),
            Weekday::Wednesday
        );
        assert_eq!(
            get_first_day_of_month(2024, Month::June).unwrap(),
            Weekday::Saturday
        );
        assert_eq!(
            get_first_day_of_month(2024, Month::July).unwrap(),
            Weekday::Monday
        );
        assert_eq!(
            get_first_day_of_month(2024, Month::August).unwrap(),
            Weekday::Thursday
        );
        assert_eq!(
            get_first_day_of_month(2024, Month::September).unwrap(),
            Weekday::Sunday
        );
        assert_eq!(
            get_first_day_of_month(2024, Month::October).unwrap(),
            Weekday::Tuesday
        );
        assert_eq!(
            get_first_day_of_month(2024, Month::November).unwrap(),
            Weekday::Friday
        );
        assert_eq!(
            get_first_day_of_month(2024, Month::December).unwrap(),
            Weekday::Sunday
        );
    }

    #[test]
    fn test_get_first_monday_of_month() {
        assert_eq!(
            get_first_monday_of_month(2024, Month::January).unwrap(),
            Date::from_calendar_date(2024, Month::January, 1).unwrap()
        );
        assert_eq!(
            get_first_monday_of_month(2024, Month::February).unwrap(),
            Date::from_calendar_date(2024, Month::February, 5).unwrap()
        );
        assert_eq!(
            get_first_monday_of_month(2024, Month::March).unwrap(),
            Date::from_calendar_date(2024, Month::March, 4).unwrap()
        );
        assert_eq!(
            get_first_monday_of_month(2024, Month::April).unwrap(),
            Date::from_calendar_date(2024, Month::April, 1).unwrap()
        );
        assert_eq!(
            get_first_monday_of_month(2024, Month::May).unwrap(),
            Date::from_calendar_date(2024, Month::May, 6).unwrap()
        );
        assert_eq!(
            get_first_monday_of_month(2024, Month::June).unwrap(),
            Date::from_calendar_date(2024, Month::June, 3).unwrap()
        );
        assert_eq!(
            get_first_monday_of_month(2024, Month::July).unwrap(),
            Date::from_calendar_date(2024, Month::July, 1).unwrap()
        );
        assert_eq!(
            get_first_monday_of_month(2024, Month::August).unwrap(),
            Date::from_calendar_date(2024, Month::August, 5).unwrap()
        );
        assert_eq!(
            get_first_monday_of_month(2024, Month::September).unwrap(),
            Date::from_calendar_date(2024, Month::September, 2).unwrap()
        );
        assert_eq!(
            get_first_monday_of_month(2024, Month::October).unwrap(),
            Date::from_calendar_date(2024, Month::October, 7).unwrap()
        );
        assert_eq!(
            get_first_monday_of_month(2024, Month::November).unwrap(),
            Date::from_calendar_date(2024, Month::November, 4).unwrap()
        );
        assert_eq!(
            get_first_monday_of_month(2024, Month::December).unwrap(),
            Date::from_calendar_date(2024, Month::December, 2).unwrap()
        );
    }
}
