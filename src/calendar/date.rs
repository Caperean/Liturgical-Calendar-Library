//! Date utilities used by the liturgical calendar.
//!
//! This module provides helper functions for working with dates,
//! including safe construction, weekday extraction, and simple
//! date arithmetic. All logic here is independent from liturgical
//! rules and is purely technical.

use chrono::{NaiveDate, Datelike, Duration};

/// Creates a `NaiveDate` from year, month, and day.
/// This function guarantees a valid date or panics with a clear message.
///
/// Example:
/// ```
/// let d = create_date(2026, 4, 12);
/// ```
pub fn create_date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day)
        .expect("Invalid date provided to create_date()")
}

/// Returns the weekday number (0 = Monday, 6 = Sunday).
pub fn weekday(date: NaiveDate) -> u32 {
    date.weekday().num_days_from_monday()
}

/// Adds a number of days to a date and returns the result.

pub fn add_days(date: NaiveDate, days: i64) -> NaiveDate {
    date + Duration::days(days)
}

/// Returns the number of days between two dates.
/// Positive if `b` is after `a`, negative otherwise.
pub fn days_between(a: NaiveDate, b: NaiveDate) -> i64 {
    (b - a).num_days()
}
