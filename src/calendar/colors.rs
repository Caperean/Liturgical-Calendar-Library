use crate::calendar::rules::{LiturgicalSeason, LiturgicalRank, LiturgicalColor};

pub fn determine_color(season: &LiturgicalSeason, rank: &LiturgicalRank) -> LiturgicalColor {
    match rank {
        LiturgicalRank::Solemnity => LiturgicalColor::White,
        LiturgicalRank::Feast => LiturgicalColor::White,
        _ => match season {
            LiturgicalSeason::Advent => LiturgicalColor::Violet,
            LiturgicalSeason::Lent => LiturgicalColor::Violet,
            LiturgicalSeason::Easter => LiturgicalColor::White,
            LiturgicalSeason::Christmas => LiturgicalColor::White,
            LiturgicalSeason::OrdinaryTime => LiturgicalColor::Green,
        }
    }
}
