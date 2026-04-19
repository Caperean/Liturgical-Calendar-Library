use crate::calendar::rules::LiturgicalRank;
use crate::calendar::fixed_feasts::FixedFeast;
use crate::calendar::movable_feasts::MovableFeast;

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

pub fn rank_for_movable_feast(_feast: &MovableFeast) -> LiturgicalRank {
    LiturgicalRank::Solemnity
}
