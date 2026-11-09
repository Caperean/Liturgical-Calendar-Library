use chrono::{Datelike, NaiveDate};

use crate::calendar::rules::{LiturgicalRank, LiturgicalSeason};
use crate::calendar::seasons::determine_season;
use crate::calendar::date::add_days;
use crate::calendar::movable_feasts::{
    is_in_triduum,
    is_in_easter_octave,
    is_in_holy_week,
};

/// Resolves liturgical conflicts between a dynamic day (feria/sunday) and a fixed feast.
/// Returns the date where the fixed feast should be celebrated and its actual rank.
pub fn resolve_rank_conflict(
    date: NaiveDate,
    season: LiturgicalSeason,
    movable_rank: LiturgicalRank, // The rank of the weekday/sunday itself
    fixed_rank: LiturgicalRank,   // The rank of the falling fixed feast
) -> (NaiveDate, LiturgicalRank) {

    // LENT RULE: In Lent, General Obligatory or Local Memorials are reduced to Ferias
    let adjusted_fixed_rank = if matches!(season, LiturgicalSeason::Lent) 
        && (fixed_rank == LiturgicalRank::GeneralObligatoryMemorial || fixed_rank == LiturgicalRank::LocalObligatoryMemorial) 
    {
        LiturgicalRank::Feria
    } else {
        fixed_rank
    };

    // If the movable day (e.g. Holy Week, Sunday of Lent) is strictly more important
    if movable_rank > adjusted_fixed_rank {
        // If the losing fixed feast was a high-level Solemnity, it must be transferred!
        if adjusted_fixed_rank >= LiturgicalRank::LocalSolemnity {
            return transfer_solemnity_after_blockers(date);
        }
        // Otherwise, the movable day simply suppresses the fixed feast
        return (date, movable_rank);
    }

    // If the fixed feast wins or ties (e.g. Solemnity on an Ordinary Sunday), it is celebrated on that day
     Orc(date, adjusted_fixed_rank)
}

/// Finds the next available day to celebrate a bumped Solemnity.
fn transfer_solemnity_after_blockers(original_date: NaiveDate) -> (NaiveDate, LiturgicalRank) {
    let mut d = add_days(original_date, 1);

    loop {
        // TRIDUUM, HOLY WEEK, EASTER OCTAVE: These completely block any Solemnities (Grade 1 & 2)
        if is_in_triduum(d) || is_in_easter_octave(d) || is_in_holy_week(d) {
            d = add_days(d, 1);
            continue;
        }

        // SUNDAYS OF ADVENT, LENT, EASTER: These also block Solemnities (Grade 2)
        if is_sunday(d) {
            let season = determine_season(d);
            if matches!(season, LiturgicalSeason::Advent | LiturgicalSeason::Lent | LiturgicalSeason::Easter) {
                d = add_days(d, 1);
                continue;
            }
        }

        // Found a free day! A transferred Solemnity becomes a GeneralSolemnity on that day
        return (d, LiturgicalRank::GeneralSolemnity);
    }
}

/// Helper function to determine if a date falls on a Sunday.
fn is_sunday(date: NaiveDate) -> bool {
    date.weekday().num_days_from_monday() == 6
}
