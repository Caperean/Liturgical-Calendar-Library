//! Core computation logic for the liturgical calendar.
//!
//! This module combines date utilities, liturgical rules,
//! and algorithms to determine the liturgical season, rank,
//! color, and other properties of a given day.

use chrono::{NaiveDate, Weekday};
use serde::Serialize;

use crate::calendar::date::{create_date, add_days};
use crate::calendar::rules::{LiturgicalSeason, LiturgicalRank, LiturgicalColor};

/// Main data structure returned by the computation layer.
#[derive(Debug, Serialize)]
pub struct LiturgicalDay {
    pub date: NaiveDate,
    pub season: LiturgicalSeason,
    pub rank: LiturgicalRank,
    pub color: LiturgicalColor,
}

/// Oblicza pierwszą niedzielę Adwentu (4 niedziele przed Bożym Narodzeniem)
fn calculate_advent_start(year: i32) -> NaiveDate {
    let christmas = NaiveDate::from_ymd_opt(year, 12, 25).unwrap();
    let weekday = christmas.weekday().num_days_from_sunday() as i64;
    let last_sunday = add_days(christmas, -(weekday));
    add_days(last_sunday, -21) // 4 niedziele wcześniej
}

/// Computes the liturgical day for a given date.
pub fn compute_liturgical_day(year: i32, month: u32, day: u32) -> LiturgicalDay {
    let date = create_date(year, month, day);

    let easter = calculate_easter(year);
    let ash_wednesday = add_days(easter, -46);
    let palm_sunday = add_days(easter, -7);
    let holy_thursday = add_days(easter, -3);
    let good_friday = add_days(easter, -2);
    let holy_saturday = add_days(easter, -1);
    let pentecost = add_days(easter, 49);

    let advent_start = calculate_advent_start(year);
    let christmas = NaiveDate::from_ymd_opt(year, 12, 25).unwrap();

    let (season, color) = if date >= advent_start && date < christmas {
        (LiturgicalSeason::Advent, LiturgicalColor::Violet)
    } else if date >= christmas && date <= add_days(christmas, 12) {
        (LiturgicalSeason::Christmas, LiturgicalColor::White)
    } else if date >= ash_wednesday && date < palm_sunday {
        (LiturgicalSeason::Lent, LiturgicalColor::Violet)
    } else if date == palm_sunday {
        (LiturgicalSeason::Lent, LiturgicalColor::Red)
    } else if date == holy_thursday {
        (LiturgicalSeason::Lent, LiturgicalColor::White)
    } else if date == good_friday {
        (LiturgicalSeason::Lent, LiturgicalColor::Red)
    } else if date == holy_saturday {
        (LiturgicalSeason::Lent, LiturgicalColor::Violet)
    } else if date == easter {
        (LiturgicalSeason::Easter, LiturgicalColor::White)
    } else if date > easter && date <= pentecost {
        (LiturgicalSeason::Easter, LiturgicalColor::White)
    } else {
        (LiturgicalSeason::OrdinaryTime, LiturgicalColor::Green)
    };

    let rank = if date == easter {
        LiturgicalRank::Solemnity
    } else {
        LiturgicalRank::Feria
    };

    LiturgicalDay {
        date,
        season,
        rank,
        color,
    }
}

/// Converts the computed liturgical day into JSON.
pub fn get_liturgical_day_json(year: i32, month: u32, day: u32) -> String {
    let day = compute_liturgical_day(year, month, day);
    serde_json::to_string(&day).unwrap()
}