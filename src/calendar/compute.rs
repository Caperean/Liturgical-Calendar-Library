use chrono::NaiveDate;
use serde::Serialize;
use serde_json::json;

use crate::calendar::date::create_date;
use crate::calendar::fixed_feasts::get_fixed_feast;
use crate::calendar::movable_feasts::get_movable_feast;
use crate::calendar::seasons::determine_season;
use crate::calendar::ranks::{rank_for_fixed_feast, rank_for_movable_feast};
use crate::calendar::colors::determine_color;

use crate::calendar::rules::{LiturgicalSeason, LiturgicalRank, LiturgicalColor};

#[derive(Debug, Serialize)]
pub struct LiturgicalDay {
    pub name: String,
    pub season: LiturgicalSeason,
    pub rank: LiturgicalRank,
    pub color: LiturgicalColor,
}



pub fn get_liturgical_day_json(year:i32, month:i32, day:i32, country:*const char) -> json::Value {
    let liturgical_day = compute_liturgical_day(year, month as u32, day as u32, country:*c_char);
    json!({
        "date": liturgical_day.date.format("%Y-%m-%d").to_string(),
        "season": format!("{:?}", liturgical_day.season),
        "rank": format!("{:?}", liturgical_day.rank),
        "color": format!("{:?}", liturgical_day.color),
    })
}

pub fn compute_liturgical_day(year: i32, month: u32, day: u32, country: &str) -> LiturgicalDay {
    let date = create_date(year, month, day);
    let season = determine_season(date);

    if let Some(feast) = get_movable_feast(date) {
        let rank = rank_for_movable_feast(&feast);
        let color = determine_color(&season, &rank);
        return LiturgicalDay { date, season, rank, color };
    }

    if date.weekday() == chrono::Weekday::Sun {
        let rank = determine_sunday_rank(&season);
        let color = determine_color(&season, &rank);
        return LiturgicalDay { date, season, rank, color };
    }

    if let Some(feast) = get_fixed_feast(date) {
        let rank = rank_for_fixed_feast(&feast);
        let color = determine_color(&season, &rank);
        return LiturgicalDay { date, season, rank, color };
    }

    let rank = LiturgicalRank::Feria;
    let color = determine_color(&season, &rank);

    LiturgicalDay { date, season, rank, color }
}

