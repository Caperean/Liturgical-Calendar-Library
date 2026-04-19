use chrono::NaiveDate;
use serde::Serialize;

use crate::calendar::date::create_date;
use crate::calendar::fixed_feasts::get_fixed_feast;
use crate::calendar::movable_feasts::get_movable_feast;
use crate::calendar::seasons::determine_season;
use crate::calendar::ranks::{rank_for_fixed_feast, rank_for_movable_feast};
use crate::calendar::colors::determine_color;

use crate::calendar::rules::{LiturgicalSeason, LiturgicalRank, LiturgicalColor};

#[derive(Debug, Serialize)]
pub struct LiturgicalDay {
    pub date: NaiveDate,
    pub season: LiturgicalSeason,
    pub rank: LiturgicalRank,
    pub color: LiturgicalColor,
}

pub fn compute_liturgical_day(year: i32, month: u32, day: u32) -> LiturgicalDay {
    let date = create_date(year, month, day);

    // 1. Movable feasts
    if let Some(feast) = get_movable_feast(date) {
        let season = determine_season(date);
        let rank = rank_for_movable_feast(&feast);
        let color = determine_color(&season, &rank);
        return LiturgicalDay { date, season, rank, color };
    }

    // 2. Fixed feasts
    if let Some(feast) = get_fixed_feast(date) {
        let season = determine_season(date);
        let rank = rank_for_fixed_feast(&feast);
        let color = determine_color(&season, &rank);
        return LiturgicalDay { date, season, rank, color };
    }

    // 3. Sundays
    if date.weekday().num_days_from_monday() == 6 {
        let season = determine_season(date);
        let rank = LiturgicalRank::Feast;
        let color = determine_color(&season, &rank);
        return LiturgicalDay { date, season, rank, color };
    }

    // 4. Feria
    let season = determine_season(date);
    let rank = LiturgicalRank::Feria;
    let color = determine_color(&season, &rank);

    LiturgicalDay { date, season, rank, color }
}
