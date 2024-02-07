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

use time::{Date, Month};

use crate::functions::{
    contains_leap_year, get_years_in_range, is_last_day_of_february, leap_year_count,
};

/// Day count conventions.
///
/// From Wikipedia (<https://en.wikipedia.org/wiki/Day_count_convention>):
/// """
/// In finance, a day count convention determines how interest accrues
/// over time for a variety of investments, including bonds, notes,
/// loans, mortgages, medium-term notes, swaps, and forward rate agreements (FRAs).
/// This determines the number of days between two coupon payments,
/// thus calculating the amount transferred on payment dates and also the
/// accrued interest for dates between payments. The day count is also
/// used to quantify periods of time when discounting a cash-flow to its
/// present value. When a security such as a bond is sold between interest
/// payment dates, the seller is eligible to some fraction of the coupon amount.
/// """
#[allow(non_camel_case_types)]
pub enum DayCountConvention {
    /// The '1/1' day count, which always returns a day count of 1.
    One_One,

    /// The 'Act/360' day count, which divides the actual number of days by 360.
    Actual_360,

    /// The 'Act/364' day count, which divides the actual number of days by 364.
    Actual_364,

    /// The 'Act/365.25' day count, which divides the actual number of days by 365.25.
    Actual_365_25,

    /// The 'Act/365 Actual' day count, which divides the actual number of days
    /// by 366 if a leap day is contained, or by 365 if not.
    Actual_365_Actual,

    /// The 'Act/365F' day count, which divides the actual number of days by 365 (fixed).
    Actual_365_Fixed,

    /// The 'Act/365L' day count, which divides the actual number of days by 365 or 366.
    Actual_365_Leap,

    /// The 'Act/Act AFB' day count, which divides the actual number of days by 366
    /// if a leap day is contained, or by 365 if not, with additional rules for periods over one year.
    Actual_Actual_AFB,

    /// The 'Act/Act ICMA' day count, which divides the actual number of days by
    /// the actual number of days in the coupon period multiplied by the frequency.
    Actual_Actual_ICMA,

    /// The 'Act/Act ISDA' day count, which divides the actual number of days in a
    /// leap year by 366 and the actual number of days in a standard year by 365.
    Actual_Actual_ISDA,

    /// The 'NL/360' day count, which divides the actual number of days omitting leap days by 360.
    NL_360,

    /// The 'NL/365' day count, which divides the actual number of days omitting leap days by 365.
    NL_365,

    /// The '30/360 ISDA' day count, which treats input day-of-month 31 specially.
    Thirty_360_ISDA,

    /// The '30E/360' day count, which treats input day-of-month 31 specially.
    Thirty_E_360,

    /// The '30E/360 ISDA' day count, which treats input day-of-month 31 and end of February specially.
    Thirty_E_360_ISDA,

    /// The '30E/365' day count, which treats input day-of-month 31 and end of February specially.
    Thirty_E_365,

    /// The '30E+/360' day count, which treats input day-of-month 31 specially.
    Thirty_E_Plus_360,

    /// The '30U/360' day count, which treats input day-of-month 31 and end of February specially.
    Thirty_U_360,
}

trait DayCounter {
    fn day_count(&self, start_date: Date, end_date: Date) -> f64;
    fn year_fraction(
        &self,
        start_date: Date,
        end_date: Date,
        convention: DayCountConvention,
    ) -> f64;
}

impl DayCountConvention {
    /// Entry point for day count factor calculation.
    #[rustfmt::skip]
    pub fn day_count_factor(&self, start_date: Date, end_date: Date) -> f64 {
        match self {
            Self::One_One => Self::day_count_factor_one_one(start_date, end_date),
            Self::Actual_360 => Self::day_count_factor_actual_360(start_date, end_date),
            Self::Actual_364 => Self::day_count_factor_actual_364(start_date, end_date),
            Self::Actual_365_25 => Self::day_count_factor_actual_365_25(start_date, end_date),
            Self::Actual_365_Actual => Self::day_count_factor_actual_365_actual(start_date, end_date),
            Self::Actual_365_Fixed => Self::day_count_factor_actual_365_fixed(start_date, end_date),
            Self::Actual_365_Leap => Self::day_count_factor_actual_365_leap(start_date, end_date),
            Self::Actual_Actual_AFB => Self::day_count_factor_actual_actual_afb(start_date, end_date),
            Self::Actual_Actual_ICMA => Self::day_count_factor_actual_actual_icma(start_date, end_date),
            Self::Actual_Actual_ISDA => Self::day_count_factor_actual_actual_isda(start_date, end_date),
            Self::NL_360 => Self::day_count_factor_nl_360(start_date, end_date),
            Self::NL_365 => Self::day_count_factor_nl_365(start_date, end_date),
            Self::Thirty_360_ISDA => Self::day_count_factor_thirty_360_isda(start_date, end_date),
            Self::Thirty_E_360 => Self::day_count_factor_thirty_e_360(start_date, end_date),
            Self::Thirty_E_360_ISDA => Self::day_count_factor_thirty_e_360_isda(start_date, end_date),
            Self::Thirty_E_365 => Self::day_count_factor_thirty_e_365(start_date, end_date),
            Self::Thirty_E_Plus_360 => Self::day_count_factor_thirty_e_plus_360(start_date, end_date),
            Self::Thirty_U_360 => Self::day_count_factor_thirty_u_360(start_date, end_date),
        }
    }

    // One/One day count factor calculation.
    fn day_count_factor_one_one(_start_date: Date, _end_date: Date) -> f64 {
        1.0
    }

    // Actual/360 day count factor calculation.
    fn day_count_factor_actual_360(start_date: Date, end_date: Date) -> f64 {
        (end_date - start_date).whole_days() as f64 / 360.0
    }

    // Actual/364 day count factor calculation.
    fn day_count_factor_actual_364(start_date: Date, end_date: Date) -> f64 {
        (end_date - start_date).whole_days() as f64 / 364.0
    }

    // Actual/365.25 day count factor calculation.
    fn day_count_factor_actual_365_25(start_date: Date, end_date: Date) -> f64 {
        (end_date - start_date).whole_days() as f64 / 365.25
    }

    // Actual/365 Actual day count factor calculation.
    fn day_count_factor_actual_365_actual(start_date: Date, end_date: Date) -> f64 {
        match contains_leap_year(start_date, end_date) {
            true => (end_date - start_date).whole_days() as f64 / 366.0,
            false => (end_date - start_date).whole_days() as f64 / 365.0,
        }
    }

    // Actual/365F day count factor calculation.
    fn day_count_factor_actual_365_fixed(start_date: Date, end_date: Date) -> f64 {
        (end_date - start_date).whole_days() as f64 / 365.0
    }

    // Actual/365L day count factor calculation.
    fn day_count_factor_actual_365_leap(start_date: Date, end_date: Date) -> f64 {
        match contains_leap_year(start_date, end_date) {
            true => (end_date - start_date).whole_days() as f64 / 366.0,
            false => (end_date - start_date).whole_days() as f64 / 365.0,
        }
    }

    // Actual/Actual AFB day count factor calculation.
    fn day_count_factor_actual_actual_afb(start_date: Date, end_date: Date) -> f64 {
        let (y1, _y2) = (start_date.year(), end_date.year());
        let (_m1, m2) = (start_date.month(), end_date.month());
        let (_d1, d2) = (start_date.day(), end_date.day());

        let stub_date = if Date::from_calendar_date(y1, m2, d2).unwrap() < start_date {
            Date::from_calendar_date(y1 + 1, m2, d2).unwrap()
        } else {
            Date::from_calendar_date(y1, m2, d2).unwrap()
        };

        let initial_stub_days = (stub_date - start_date).whole_days() as f64;
        // let final_stub_days = (end_date - stub_date).whole_days() as f64;
        let final_stub_years = (end_date.year() - stub_date.year()) as f64;
        let initial_stub_contains_leap = contains_leap_year(start_date, stub_date);

        match initial_stub_contains_leap {
            true => final_stub_years + initial_stub_days / 366.0,
            false => (end_date - start_date).whole_days() as f64 / 365.0,
        }
    }

    // Actual/Actual ICMA day count factor calculation.
    fn day_count_factor_actual_actual_icma(_start_date: Date, _end_date: Date) -> f64 {
        todo!()
    }

    // Actual/Actual ISDA day count factor calculation.
    fn day_count_factor_actual_actual_isda(start_date: Date, end_date: Date) -> f64 {
        let years = get_years_in_range(start_date, end_date);
        let year_day_counts = get_years_in_range(start_date, end_date);

        let start_day_count = (Date::from_calendar_date(start_date.year(), Month::December, 31)
            .unwrap()
            - start_date)
            .whole_days() as f64;

        let end_day_count = (end_date
            - Date::from_calendar_date(end_date.year(), Month::January, 1).unwrap())
        .whole_days() as f64;

        let mut dcf = 0.0;

        for _ in 1..years.len() {
            dcf += 1.0;
        }

        dcf + start_day_count / year_day_counts[0] as f64
            + end_day_count / year_day_counts[years.len() - 1] as f64
    }

    // NL/360 day count factor calculation.
    fn day_count_factor_nl_360(start_date: Date, end_date: Date) -> f64 {
        let day_count = (end_date - start_date).whole_days() as f64;
        let leap_years = leap_year_count(start_date, end_date) as f64;

        (day_count - leap_years) / 360.0
    }

    // NL/365 day count factor calculation.
    fn day_count_factor_nl_365(start_date: Date, end_date: Date) -> f64 {
        let day_count = (end_date - start_date).whole_days() as f64;
        let leap_years = leap_year_count(start_date, end_date) as f64;

        (day_count - leap_years) / 365.0
    }

    // 30/360 ISDA day count factor calculation.
    fn day_count_factor_thirty_360_isda(start_date: Date, end_date: Date) -> f64 {
        let (y1, m1, mut d1) = Self::thirty_360_unpack_date(start_date);
        let (y2, m2, mut d2) = Self::thirty_360_unpack_date(end_date);

        if d1 == 31 {
            d1 = 30;
        }

        if d1 == 30 && d2 == 31 {
            d2 = 30;
        }

        Self::thirty_360_numerator(y1, y2, m1, m2, d1, d2) / 360.0
    }

    // 30E/360 day count factor calculation.
    fn day_count_factor_thirty_e_360(start_date: Date, end_date: Date) -> f64 {
        let (y1, m1, mut d1) = Self::thirty_360_unpack_date(start_date);
        let (y2, m2, mut d2) = Self::thirty_360_unpack_date(end_date);

        if d1 == 31 {
            d1 = 30;
        }

        if d2 == 31 {
            d2 = 30;
        }

        Self::thirty_360_numerator(y1, y2, m1, m2, d1, d2) / 360.0
    }

    // 30E/360 ISDA day count factor calculation.
    fn day_count_factor_thirty_e_360_isda(start_date: Date, end_date: Date) -> f64 {
        let (y1, m1, mut d1) = Self::thirty_360_unpack_date(start_date);
        let (y2, m2, mut d2) = Self::thirty_360_unpack_date(end_date);

        if d1 == 31 || is_last_day_of_february(start_date) {
            d1 = 30;
        }

        if d2 == 31 || is_last_day_of_february(end_date) {
            d2 = 30;
        }

        Self::thirty_360_numerator(y1, y2, m1, m2, d1, d2) / 360.0
    }

    // 30E/365 day count factor calculation.
    fn day_count_factor_thirty_e_365(start_date: Date, end_date: Date) -> f64 {
        let (y1, m1, mut d1) = Self::thirty_360_unpack_date(start_date);
        let (y2, m2, mut d2) = Self::thirty_360_unpack_date(end_date);

        if d1 == 31 || is_last_day_of_february(start_date) {
            d1 = 30;
        }

        if d2 == 31 || is_last_day_of_february(end_date) {
            d2 = 30;
        }

        Self::thirty_360_numerator(y1, y2, m1, m2, d1, d2) / 365.0
    }

    // 30E+/360 day count factor calculation.
    fn day_count_factor_thirty_e_plus_360(start_date: Date, end_date: Date) -> f64 {
        let (y1, m1, mut d1) = Self::thirty_360_unpack_date(start_date);
        let (mut y2, mut m2, mut d2) = Self::thirty_360_unpack_date(end_date);

        if d1 == 31 {
            d1 = 30;
        }

        if d2 == 31 {
            (y2, m2, d2) = Self::thirty_360_unpack_date(end_date.next_day().unwrap());
        }

        Self::thirty_360_numerator(y1, y2, m1, m2, d1, d2) / 360.0
    }

    // 30U/360 day count factor calculation.
    fn day_count_factor_thirty_u_360(start_date: Date, end_date: Date) -> f64 {
        let (y1, m1, mut d1) = Self::thirty_360_unpack_date(start_date);
        let (y2, m2, mut d2) = Self::thirty_360_unpack_date(end_date);

        if d1 == 31 || is_last_day_of_february(start_date) {
            d1 = 30;
        }

        if d2 == 31 && d1 == 30 || is_last_day_of_february(end_date) {
            d2 = 30;
        }

        Self::thirty_360_numerator(y1, y2, m1, m2, d1, d2) / 360.0
    }

    /// Function to comput the 30/360 numerator.
    fn thirty_360_numerator(y1: i32, y2: i32, m1: i32, m2: i32, d1: i32, d2: i32) -> f64 {
        (360 * (y2 - y1) + 30 * (m2 - m1) + (d2 - d1)) as f64
    }

    /// Function to unpack the date components for 30/360 calculation.
    fn thirty_360_unpack_date(date: Date) -> (i32, i32, i32) {
        (date.year(), date.month() as i32, date.day() as i32)
    }
}

// impl<C: Calendar> DayCounter for C {
//     fn day_count(&self, start_date: Date, end_date: Date) -> f64 {
//         todo!()
//     }

//     fn year_fraction(
//         &self,
//         start_date: Date,
//         end_date: Date,
//         convention: DayCountConvention,
//     ) -> f64 {
//         let total_days = (end_date - start_date).whole_days() as f64;
//         let business_days = self.all_business_days_between(start_date, end_date).len() as f64;

//         match convention {
//             DayCountConvention::OneOne => 1.0,
//             DayCountConvention::Actual360 => total_days / 360.0,
//             DayCountConvention::Actual365 => total_days / 365.0,
//             DayCountConvention::Actual364 => total_days / 364.0,
//             DayCountConvention::ActualActualISDA => 0.0,
//             _ => 0.0,
//         }
//     }
// }
