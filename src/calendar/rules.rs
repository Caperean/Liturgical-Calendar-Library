use serde::Serialize;

/// Precedence of liturgical days, from lowest to highest.
///
/// Derived `Ord` follows the discriminants, so a higher rank always compares greater.
/// Based on the Table of Liturgical Days in the Universal Norms on the Liturgical Year
/// and the General Roman Calendar (n. 59); table row numbers are given in brackets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum LiturgicalRank {

    Feria = 1,
    OptionalMemorial,
    LocalObligatoryMemorial,
    GeneralObligatoryMemorial,
    PrivilegedFeria,

    LocalFeast,
    GeneralFeast,
    OrdinarySunday,
    GeneralLordFeast,

    LocalSolemnity,
    GeneralSolemnity,
    PrimarySolemnityAndSunday,
    Triduum,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum LiturgicalColor {
    White,
    Red,
    Green,
    Violet,
    Rose,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum LiturgicalSeason {
    Advent,
    Christmas,
    Lent,
    /// Holy Thursday (evening Mass) through Holy Saturday; separate from both Lent and Easter Time.
    Triduum,
    Easter,
    OrdinaryTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Feast {
    pub name: &'static str,
    pub rank: LiturgicalRank,
    pub color: LiturgicalColor,
    pub season: LiturgicalSeason,
}