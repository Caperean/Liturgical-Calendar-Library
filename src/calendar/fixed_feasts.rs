

use chrono::NaiveDate;

use crate::calendar::fixed_feasts::FixedFeast;
use crate::calendar::rules::LiturgicalRank;

#[derive(Debug, Clone)]
pub enum FixedFeast {
    MaryMotherOfGod,          // Jan 1
    Epiphany,                 // Jan 6
    PresentationOfTheLord,    // Feb 2
    Annunciation,             // Mar 25
    NativityOfJohnTheBaptist, // Jun 24
    PeterAndPaul,             // Jun 29
    Assumption,               // Aug 15
    AllSaints,                // Nov 1
    ImmaculateConception,     // Dec 8
    Christmas,                // Dec 25
}

pub fn get_fixed_feast(date: NaiveDate) -> Option<FixedFeast> {
    match (date.month(), date.day()) {
        (1, 1)   => Some(FixedFeast::MaryMotherOfGod),
        (1, 6)   => Some(FixedFeast::Epiphany),
        (2, 2)   => Some(FixedFeast::PresentationOfTheLord),
        (3, 25)  => Some(FixedFeast::Annunciation),
        (6, 24)  => Some(FixedFeast::NativityOfJohnTheBaptist),
        (6, 29)  => Some(FixedFeast::PeterAndPaul),
        (8, 15)  => Some(FixedFeast::Assumption),
        (11, 1)  => Some(FixedFeast::AllSaints),
        (12, 8)  => Some(FixedFeast::ImmaculateConception),
        (12, 25) => Some(FixedFeast::Christmas),
        _ => None,
    }
}

pub fn rank_for_fixed_feast(feast: &FixedFeast) -> LiturgicalRank {
    match feast {
        FixedFeast::Christmas => LiturgicalRank::Solemnity,
        FixedFeast::MaryMotherOfGod => LiturgicalRank::Solemnity,
        FixedFeast::Epiphany => LiturgicalRank::Solemnity,
        FixedFeast::PresentationOfTheLord => LiturgicalRank::Feast,
        FixedFeast::Annunciation => LiturgicalRank::Solemnity,
        FixedFeast::NativityOfJohnTheBaptist => LiturgicalRank::Solemnity,
        FixedFeast::PeterAndPaul => LiturgicalRank::Solemnity,
        FixedFeast::Assumption => LiturgicalRank::Solemnity,
        FixedFeast::AllSaints => LiturgicalRank::Solemnity,
        FixedFeast::ImmaculateConception => LiturgicalRank::Solemnity,
    }
}
