use chrono::NaiveDate;

use crate::calendar::rules::{LiturgicalRank, LiturgicalSeason};
use crate::calendar::seasons::determine_season;
use crate::calendar::date::add_days;
use crate::calendar::movable_feasts::{
    is_in_triduum,
    is_in_easter_octave,
    is_in_holy_week,
};


pub fn resolve_rank_conflict(
    date: NaiveDate,
    season: LiturgicalSeason,
    rank: LiturgicalRank,
) -> (NaiveDate, LiturgicalRank) {

    if is_in_triduum(date) || is_in_easter_octave(date) {
        return (date, rank);
    }

    let is_sunday = is_sunday(date);
    let mut final_date = date;
    let mut final_rank = rank;

    
    if matches!(season, LiturgicalSeason::Lent) {
        match rank {
            LiturgicalRank::Memorial | LiturgicalRank::OptionalMemorial => {
                final_rank = LiturgicalRank::Feria;
            }
            _ => {}
        }
    }

    
    
    if is_sunday {
        match season {
            LiturgicalSeason::Advent
            | LiturgicalSeason::Lent
            | LiturgicalSeason::Easter => {
                if rank == LiturgicalRank::Solemnity {
                
                    let (t_date, t_rank) = transfer_solemnity_after_blockers(date);
                    return (t_date, t_rank);
                } else if rank.priority() < LiturgicalRank::Sunday.priority() {
               
                    final_rank = LiturgicalRank::Sunday;
                    return (final_date, final_rank);
                }
            }
            _ => {}
        }
    }

  
    if is_sunday && matches!(season, LiturgicalSeason::OrdinaryTime | LiturgicalSeason::Christmas) {
        match rank {
            LiturgicalRank::Solemnity => {
            
                final_rank = LiturgicalRank::Solemnity;
                return (final_date, final_rank);
            }
            _ => {
                if rank.priority() < LiturgicalRank::Sunday.priority() {
                    final_rank = LiturgicalRank::Sunday;
                    return (final_date, final_rank);
                }
            }
        }
    }


    if is_sunday {
        if rank.priority() < LiturgicalRank::Sunday.priority() {
            final_rank = LiturgicalRank::Sunday;
        }
    }

    (final_date, final_rank)
}


fn transfer_solemnity_after_blockers(original_date: NaiveDate) -> (NaiveDate, LiturgicalRank) {
    let mut d = add_days(original_date, 1);

    loop {
      
        if is_in_triduum(d) || is_in_easter_octave(d) || is_in_holy_week(d) {
            d = add_days(d, 1);
            continue;
        }

        
        if is_sunday(d) {
            let season = determine_season(d);
            match season {
                LiturgicalSeason::Advent
                | LiturgicalSeason::Lent
                | LiturgicalSeason::Easter => {
                    d = add_days(d, 1);
                    continue;
                }
                _ => {}
            }
        }

       
        return (d, LiturgicalRank::Solemnity);
    }
}


fn is_sunday(date: NaiveDate) -> bool {
    // chrono: Monday = 0, Sunday = 6
    date.weekday().num_days_from_monday() == 6
}
