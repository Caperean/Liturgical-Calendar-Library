use chrono::NaiveDate;
use crate::calendar::rules::{LiturgicalRank, LiturgicalSeason};
use crate::calendar::date::add_days;

pub fn resolve_rank_conflict(
    date: NaiveDate,
    season: LiturgicalSeason,
    rank: LiturgicalRank,
) -> (NaiveDate, LiturgicalRank) {

    let is_sunday = date.weekday().num_days_from_monday() == 6;

    // 1. Triduum i Wielki Post – nie nadpisujemy (masz to w movable_feasts)
    if season == LiturgicalSeason::Lent {
        return (date, rank);
    }

    // 2. Uroczystość w niedzielę Adwentu, Wielkiego Postu, Wielkanocy → przeniesienie
    if is_sunday {
        match season {
            LiturgicalSeason::Advent |
            LiturgicalSeason::Lent |
            LiturgicalSeason::Easter => {
                if rank == LiturgicalRank::Solemnity {
                    let new_date = add_days(date, 1);
                    return (new_date, LiturgicalRank::Solemnity);
                }
            }
            _ => {}
        }
    }

    // 3. Niedziela > święto/wspomnienie
    if is_sunday {
        match rank {
            LiturgicalRank::Solemnity => (date, LiturgicalRank::Solemnity),
            _ => (date, LiturgicalRank::Feast),
        }
    } else {
        (date, rank)
    }
}
