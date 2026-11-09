

use chrono::NaiveDate;

use crate::calendar::fixed_feasts::FixedFeast;
use crate::calendar::rules::LiturgicalRank;
use crate::calendar::seasons::LiturgicalSeason;



 fn DS(date: NaiveDate) -> LiturgicalSeason {
    determine_season(date) 
}



pub fn get_fixed_feast(date: NaiveDate) -> Vec<Feast> {
    let mut feasts = Vec::new();
    match (date.month(), date.day()) {
        (1, 1)   => feasts.push(Feast { name: "Mary, Mother of God", rank: Solemnity, color: White, season: DS(date) }),
        (1, 6)   => feasts.push(Feast { name: "Epiphany", rank: Solemnity, color: White, season: DS(date) }),
        (2, 2)   => feasts.push(Feast { name: "Presentation of the Lord", rank: Feast, color: White, season: DS(date) }),
        (3, 25)  => feasts.push(Feast { name: "Annunciation", rank: Solemnity, color: White, season: DS(date) }),
        (6, 24)  => feasts.push(Feast { name: "Nativity of John the Baptist", rank: Solemnity, color: White, season: DS(date) }),
        (6, 29)  => feasts.push(Feast { name: "Saint Peter and Saint Paul", rank: Solemnity, color: Red, season: DS(date) }),
        (8, 15)  => feasts.push(Feast { name: "Assumption", rank: Solemnity, color: White, season: DS(date) }),
        (11, 1)  => feasts.push(Feast { name: "All Saints", rank: Solemnity, color: White, season: DS(date) }),
        (12, 8)  => feasts.push(Feast { name: "Immaculate Conception", rank: Solemnity, color: White, season: DS(date) }),
        (12, 25) => feasts.push(Feast { name: "Christmas", rank: Solemnity, color: White, season: DS(date) }),
        _ => (),
    }
    feasts
}


