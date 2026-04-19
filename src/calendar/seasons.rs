use chrono::NaiveDate;
use crate::calendar::date::add_days;
use crate::calendar::easter::calculate_easter;
use crate::calendar::rules::LiturgicalSeason;

pub fn determine_season(date: NaiveDate) -> LiturgicalSeason {
    let year = date.year();
    let easter = calculate_easter(year);
    let ash_wednesday = add_days(easter, -46);
    let palm_sunday = add_days(easter, -7);
    let pentecost = add_days(easter, 49);

    let christmas = NaiveDate::from_ymd_opt(year, 12, 25).unwrap();
    let advent_start = calculate_advent_start(year);

    if date >= advent_start && date < christmas {
        LiturgicalSeason::Advent
    } else if date >= christmas && date <= add_days(christmas, 12) {
        LiturgicalSeason::Christmas
    } else if date >= ash_wednesday && date < palm_sunday {
        LiturgicalSeason::Lent
    } else if date >= easter && date <= pentecost {
        LiturgicalSeason::Easter
    } else {
        LiturgicalSeason::OrdinaryTime
    }
}

fn calculate_advent_start(year: i32) -> NaiveDate {
    let christmas = NaiveDate::from_ymd_opt(year, 12, 25).unwrap();
    let weekday = christmas.weekday().num_days_from_sunday() as i64;
    let last_sunday = add_days(christmas, -weekday);
    add_days(last_sunday, -21)
}
