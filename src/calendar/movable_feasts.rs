
use chrono::NaiveDate;
use crate::calendar::easter::calculate_easter;
use crate::calendar::date::add_days;

use crate::calendar::movable_feasts::MovableFeast;



fn DS(date: NaiveDate) -> LiturgicalSeason {
    determine_season(date) 
}


pub fn get_movable_feast(date: NaiveDate) -> Vec<Feast> {

    let mut feasts = Vec::new();

    let year = date.year();
    let easter = calculate_easter(year);

    match (date.month(), date.day())
    {
        (add_days(easter, -7).month(), add_days(easter, -7).day()) => 
        feasts.push(Feast { name: "Palm Sunday", rank: Solemnity, color: Red, season: DS(date) }),

        (add_days(easter, -3).month(), add_days(easter, -3).day()) => 
        feasts.push(Feast { name: "Holy Thursday", rank: Solemnity, color: White, season: DS(date) }),

        (add_days(easter, -2).month(), add_days(easter, -2).day()) => 
        feasts.push(Feast { name: "Good Friday", rank: Solemnity, color: Red, season: DS(date) }),

        (add_days(easter, -1).month(), add_days(easter, -1).day()) => 
        feasts.push(Feast { name: "Holy Saturday", rank: Solemnity, color: White, season: DS(date) }),

        (easter.month(), easter.day()) => 
        feasts.push(Feast { name: "Easter Sunday", rank: Solemnity, color: White, season: DS(date) }),

        (add_days(easter, 7).month(), add_days(easter, 7).day()) => 
        feasts.push(Feast { name: "Easter Octave", rank: Solemnity, color: White, season: DS(date) }),

        (add_days(easter, 39).month(), add_days(easter, 39).day()) => 
        feasts.push(Feast { name: "Ascension", rank: Solemnity, color: White, season: DS(date) }),

        (add_days(easter, 49).month(), add_days(easter, 49).day()) => 
        feasts.push(Feast { name: "Pentecost", rank: Solemnity, color: Red, season: DS(date) }),

        (add_days(easter, 56).month(), add_days(easter, 56).day()) => 
        feasts.push(Feast { name: "Trinity Sunday", rank: Solemnity, color: White, season: DS(date) }),

        (add_days(easter, 60).month(), add_days(easter, 60).day()) => 
        feasts.push(Feast { name: "Corpus Christi", rank: Solemnity, color: White, season: DS(date) }),
        _ => (),
    }

    
}