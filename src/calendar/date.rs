

use chrono::{NaiveDate, Datelike, Duration};


pub fn create_date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day)
        .expect("Invalid date provided to create_date()")
}


pub fn weekday(date: NaiveDate) -> u32 {
    date.weekday().num_days_from_monday()
}


pub fn add_days(date: NaiveDate, days: i64) -> NaiveDate {
    date + Duration::days(days)
}

pub fn days_between(a: NaiveDate, b: NaiveDate) -> i64 {
    (b - a).num_days()
}
pub fn is_in_triduum(date: NaiveDate) -> bool {
    let easter = easter_sunday(date.year());
    let holy_thursday = add_days(easter, -3);
    let good_friday = add_days(easter, -2);
    let holy_saturday = add_days(easter, -1);

    date == holy_thursday || date == good_friday || date == holy_saturday
}

pub fn is_in_easter_octave(date: NaiveDate) -> bool {
    let easter = easter_sunday(date.year());
    let octave_end = add_days(easter, 7);
    date >= easter && date <= octave_end
}


pub fn is_in_holy_week(date: NaiveDate) -> bool {
    let easter = easter_sunday(date.year());
    let palm_sunday = add_days(easter, -7);
    let holy_saturday = add_days(easter, -1);

    date >= palm_sunday && date <= holy_saturday
}
