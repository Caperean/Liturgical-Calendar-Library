use std::ffi::{CString, CStr};
use std::os::raw::c_char;

use crate::calendar::compute::get_liturgical_day_json;

/// Converts a Rust `String` into a raw C-compatible string pointer.
/// The returned pointer must be freed by calling `free_string()`.
fn string_to_c_char(s: String) -> *mut c_char {
    CString::new(s).unwrap().into_raw()
}

/// Frees memory allocated by `string_to_c_char()`.
/// This must be called by the foreign language (C/C++/Swift/Kotlin/etc.)
/// after it finishes using the returned string.
///
/// Safety:
/// - Passing a null pointer is allowed (no-op).
/// - Passing an invalid pointer results in undefined behavior.
#[no_mangle]
pub extern "C" fn free_string(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        // Reconstruct CString and let it drop, freeing the memory.
        drop(CString::from_raw(ptr));
    }
}

/// FFI entry point: returns a JSON string describing the liturgical day.
/// The caller receives a heap-allocated C string and must free it using `free_string()`.
///
/// Parameters:
/// - `year`: full year (e.g., 2026)
/// - `month`: 1–12
/// - `day`: 1–31
///
/// Returns:
/// - A pointer to a C string containing JSON data.
///   The caller is responsible for freeing the returned pointer.
#[no_mangle]
pub extern "C" fn get_liturgical_day(
    year: i32,
    month: u32,
    day: u32,
) -> *mut c_char {
    let json = get_liturgical_day_json(year, month, day);
    string_to_c_char(json)
}
