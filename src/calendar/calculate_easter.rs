//! Easter calculation module (Gregorian calendar)

use chrono::NaiveDate;

/// Computes the date of Easter Sunday for a given year (Gregorian calendar).
///
/// Algorithm: Gauss (Meeus/Jones/Butcher)
pub fn calculate_easter(year: i32) -> NaiveDate {
    let a = year % 19;
    let b = year / 100;
    let c = year % 100;
    let d = b / 4;
    let e = b % 4;
    let f = (b + 8) / 25;
    let g = (b - f + 1) / 3;
    let h = (19 * a + b - d - g + 15) % 30;
    let i = c / 4;
    let k = c % 4;
    let l = (32 + 2 * e + 2 * i - h - k) % 7;
    let m = (a + 11 * h + 22 * l) / 451;

    let month = (h + l - 7 * m + 114) / 31;
    let day = ((h + l - 7 * m + 114) % 31) + 1;

    NaiveDate::from_ymd_opt(year, month as u32, day as u32)
        .expect("Invalid Easter date computed")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easter_dates() {
        assert_eq!(calculate_easter(2024), NaiveDate::from_ymd_opt(2024, 3, 31).unwrap());
        assert_eq!(calculate_easter(2025), NaiveDate::from_ymd_opt(2025, 4, 20).unwrap());
        assert_eq!(calculate_easter(2026), NaiveDate::from_ymd_opt(2026, 4, 5).unwrap());
        assert_eq!(calculate_easter(2030), NaiveDate::from_ymd_opt(2030, 4, 21).unwrap());
    }
}