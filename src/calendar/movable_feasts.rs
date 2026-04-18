
use chrono::NaiveDate;
use crate::calendar::easter::calculate_easter;
use crate::calendar::date::add_days;

use crate::calendar::movable_feasts::MovableFeast;


/// Universal movable feasts of the Roman Catholic Church.
#[derive(Debug, Clone)]
pub enum MovableFeast {
    PalmSunday,
    HolyThursday,
    GoodFriday,
    HolySaturday,
    EasterSunday,
    EasterOctave,
    Ascension,
    Pentecost,
    TrinitySunday,
    CorpusChristi,
}


pub fn get_movable_feast(date: NaiveDate) -> Option<MovableFeast> {
    let year = date.year();
    let easter = calculate_easter(year);

    let palm_sunday      = add_days(easter, -7);
    let holy_thursday    = add_days(easter, -3);
    let good_friday      = add_days(easter, -2);
    let holy_saturday    = add_days(easter, -1);
    let easter_octave    = add_days(easter, 7);
    let ascension        = add_days(easter, 39); 
    let pentecost        = add_days(easter, 49);
    let trinity_sunday   = add_days(easter, 56);
    let corpus_christi   = add_days(easter, 60);

    match date {
        d if d == palm_sunday    => Some(MovableFeast::PalmSunday),
        d if d == holy_thursday  => Some(MovableFeast::HolyThursday),
        d if d == good_friday    => Some(MovableFeast::GoodFriday),
        d if d == holy_saturday  => Some(MovableFeast::HolySaturday),
        d if d == easter         => Some(MovableFeast::EasterSunday),
        d if d == easter_octave  => Some(MovableFeast::EasterOctave),
        d if d == ascension      => Some(MovableFeast::Ascension),
        d if d == pentecost      => Some(MovableFeast::Pentecost),
        d if d == trinity_sunday => Some(MovableFeast::TrinitySunday),
        d if d == corpus_christi => Some(MovableFeast::CorpusChristi),
        _ => None,
    }
}

pub fn rank_for_movable_feast(feast: &MovableFeast) -> LiturgicalRank {
    match feast {
        MovableFeast::EasterSunday => LiturgicalRank::Solemnity,
        MovableFeast::PalmSunday => LiturgicalRank::Solemnity,
        MovableFeast::HolyThursday => LiturgicalRank::Solemnity,
        MovableFeast::GoodFriday => LiturgicalRank::Solemnity,
        MovableFeast::HolySaturday => LiturgicalRank::Solemnity,
        MovableFeast::EasterOctave => LiturgicalRank::Solemnity,
        MovableFeast::Ascension => LiturgicalRank::Solemnity,
        MovableFeast::Pentecost => LiturgicalRank::Solemnity,
        MovableFeast::TrinitySunday => LiturgicalRank::Solemnity,
        MovableFeast::CorpusChristi => LiturgicalRank::Solemnity,
    }
}
