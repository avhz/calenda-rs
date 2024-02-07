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

use crate::constants::{
    ANNUALLY, BI_WEEKLY, DAILY, MONTHLY, QUARTERLY, SEMI_ANNUALLY, SEMI_MONTHLY, SEMI_QUARTERLY,
    TRI_ANNUALLY, WEEKLY,
};

/// Interest/coupon frequency per year.
/// This is important in finance, as it determines the number of times
/// a cash flow is paid in a year, and thus affects the present value
/// of the cash flows.
#[derive(Debug, Clone, Copy)]
pub enum Frequency {
    /// Daily.
    Daily = DAILY,

    /// Weekly.
    Weekly = WEEKLY,

    /// Bi-weekly.
    BiWeekly = BI_WEEKLY,

    /// Semi-monthly.
    SemiMonthly = SEMI_MONTHLY,

    /// Monthly.
    Monthly = MONTHLY,

    /// Semi-quarterly.
    SemiQuarterly = SEMI_QUARTERLY,

    /// Quarterly.
    Quarterly = QUARTERLY,

    /// Tri-annually.
    TriAnnually = TRI_ANNUALLY,

    /// Semi-annually.
    SemiAnnually = SEMI_ANNUALLY,

    /// Annually.
    Annually = ANNUALLY,
}
