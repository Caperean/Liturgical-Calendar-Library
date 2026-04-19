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
    Feast,
    Memorial,
    OptionalMemorial,
    Feria,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum LiturgicalColor {
    White,
    Red,
    Green,
    Violet,
    Rose,
}
