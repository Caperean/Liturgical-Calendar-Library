use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum LiturgicalSeason {
    Advent,
    Christmas,
    Lent,
    Easter,
    OrdinaryTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum LiturgicalRank {
    Solemnity,
    Sunday
    Feast,
    Memorial,
    OptionalMemorial,
    Feria,
}
impl LiturgicalRank {
    pub fn priority(&self) -> u8 {
        match self {
            LiturgicalRank::Solemnity => 6,
            LiturgicalRank::Sunday => 5,
            LiturgicalRank::Feast => 4,
            LiturgicalRank::Memorial => 3,
            LiturgicalRank::OptionalMemorial => 2,
            LiturgicalRank::Feria => 1,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum LiturgicalColor {
    White,
    Red,
    Green,
    Violet,
    Rose,
}
