use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use chrono::{Datelike, NaiveDate};

use crate::calendar::compute::get_liturgical_day_json;

/// Opaque struct representation for the Calendar type.
/// Used as a pointer handle across the FFI boundary.
pub struct Calendar;

/// Converts a Rust `String` into a raw C-compatible string pointer.
/// The returned pointer must be freed by calling `RC_free_string()`.
fn string_to_c_char(s: String) -> *mut c_char {
    CString::new(s).unwrap().into_raw()
}

/// Frees a C-compatible string pointer allocated by Rust.
#[no_mangle]
pub extern "C" fn RC_free_string(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        // Reconstruct CString and let it drop, freeing the memory.
        drop(CString::from_raw(ptr));
    }
}

/// Retrieves the liturgical data for a specific day as a JSON formatted C-string.
/// The returned pointer must be freed by calling `RC_free_string()`.
/// Returns a null pointer if the country string is invalid or cannot be parsed.
#[no_mangle]
pub unsafe extern "C" fn getDayCalendar(
    year: i32, 
    month: i32, 
    day: i32, 
    country: *const c_char
) -> *mut c_char {
    // 1. Validate the incoming input pointer
    if country.is_null() {
        return std::ptr::null_mut();
    }

    // 2. Convert the raw C-string into a safe Rust string slice (&str)
    let c_str_country = CStr::from_ptr(country);
    let country_str = match c_str_country.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(), // Return null on invalid string encoding
    };

    // 3. Invoke internal domain logic from the calendar module
    let json_response = get_liturgical_day_json(year, month, day, country_str);

    // 4. Convert the Rust String into a C-compatible pointer and transfer ownership
    string_to_c_char(json_response)
}

/// Future implementation to retrieve full calendar data for a specific year.
/// The returned pointer must be freed by calling `RC_freeCalendar()`.
#[no_mangle]
pub extern "C" fn getYearCalendar(_year: i32, _country: *const c_char) -> *mut Calendar {
    std::ptr::null_mut()
}

/// Future implementation to retrieve full calendar data for a specific month.
/// The returned pointer must be freed by calling `RC_freeCalendar()`.
#[no_mangle]
pub extern "C" fn getMonthCalendar(_year: i32, _month: i32, _country: *const c_char) -> *mut Calendar {
    std::ptr::null_mut()
}

/// Frees the heap memory associated with the Calendar pointer handle.
#[no_mangle]
pub extern "C" fn RC_freeCalendar(ptr: *mut Calendar) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        // Reconstruct the Box from the raw pointer and drop it to free heap memory
        drop(Box::from_raw(ptr));
    }
}

/// Checks whether a specific country code (ISO Alpha-2) is supported by RCalendar.
#[no_mangle]
pub unsafe extern "C" fn _RC_isCountrySupported(country: *const c_char) -> bool {
    if country.is_null() {
        return false;
    }
    
    let c_str_country = CStr::from_ptr(country);
    let country_str = match c_str_country.to_str() {
        Ok(s) => s.to_uppercase(),
        Err(_) => return false,
    };

    // TODO: Connect to your actual validation registry
    country_str == "PL" || country_str == "DE" || country_str == "US"
}

/// Retrieves a JSON-formatted array containing all supported ISO country codes.
/// The returned pointer must be freed by calling `RC_free_string()`.
#[no_mangle]
pub extern "C" fn RC_getSupportedCountries() -> *mut c_char {
    let json_response = r#"["PL", "DE", "US"]"#.to_string();
    string_to_c_char(json_response)
}

/// Returns the current Semantic Version (SemVer) of the RCalendar library.
/// The returned pointer must be freed by calling `RC_free_string()`.
#[no_mangle]
pub extern "C" fn RC_getVersion() -> *mut c_char {
    let version = env!("CARGO_PKG_VERSION").to_string();
    string_to_c_char(version)
}

/// Helper function to validate if a given date is physically valid.
#[no_mangle]
pub extern "C" fn RC_isValidDate(year: i32, month: i32, day: i32) -> bool {
    NaiveDate::from_ymd_opt(year, month as u32, day as u32).is_some()
}

/// Returns the total number of days in a specific month for a given year.
/// Returns 0 if the input year or month parameters are out of bounds.
#[no_mangle]
pub extern "C" fn getDaysInMonth(year: i32, month: i32) -> i32 {
    let current_month_start = NaiveDate::from_ymd_opt(year, month as u32, 1);
    
    match current_month_start {
        Some(date) => {
            let next_month = if month == 12 { 1 } else { month + 1 };
            let next_year = if month == 12 { year + 1 } else { year };
            
            if let Some(next_month_start) = NaiveDate::from_ymd_opt(next_year, next_month as u32, 1) {
                let duration = next_month_start.signed_duration_since(date);
                duration.num_days() as i32
            } else {
                0
            }
        }
        None => 0,
    }
}

/// Determines the day of the week for a specific date.
/// Returns: 1 = Monday, 2 = Tuesday, ..., 7 = Sunday. Returns 0 if invalid.
#[no_mangle]
pub extern "C" fn getDayOfWeek(year: i32, month: i32, day: i32) -> i32 {
    match NaiveDate::from_ymd_opt(year, month as u32, day as u32) {
        Some(date) => date.weekday().number_from_monday() as i32,
        None => 0,
    }
}

/// Checks if a given year is a leap year according to the Gregorian calendar.
#[no_mangle]
pub extern "C" fn RC_isLeapYear(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
