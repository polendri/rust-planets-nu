/*!
Contains data structures and helper methods useful to builder modules.
*/
extern crate serialize;

use self::serialize::json;
use std::char;
use std::collections;

use error;

// Public

/// Represents a 3-byte RGB colour value.
#[deriving(Eq, PartialEq, Show)]
pub struct RGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
// TODO: move RBG into a public 'data' module.
// TODO: function to convert an RGB back to a string.

/// Converts a string of the form "#XXXXXX" (where X is a hex digit) to an RGB object.
pub fn to_rgb(rgb_str: &str) -> Result<RGB, error::Error> {
    let str_to_err = |string| { error::Error::new(error::LibError, string) };
    let short_err = str_to_err("RGB string is too short.".to_string());
    let long_err = str_to_err("RGB string is too long.".to_string());
    let digit_err = str_to_err("Encountered a non-hex digit.".to_string());
    let len = rgb_str.len();
    if len < 7 {
        return Err(short_err);
    } else if len > 7 {
        return Err(long_err);
    }
    let mut iter = rgb_str.chars();
    match iter.next().unwrap() {
        '#' => (),
        _ => return Err(str_to_err("Could not find leading '#'.".to_string())),
    };
    let r = (expect!(char::to_digit(iter.next().unwrap(), 16), digit_err) as u8 << 4) +
        expect!(char::to_digit(iter.next().unwrap(), 16), digit_err) as u8;
    let g = (expect!(char::to_digit(iter.next().unwrap(), 16), digit_err) as u8 << 4) +
        expect!(char::to_digit(iter.next().unwrap(), 16), digit_err) as u8;
    let b = (expect!(char::to_digit(iter.next().unwrap(), 16), digit_err) as u8 << 4) +
        expect!(char::to_digit(iter.next().unwrap(), 16), digit_err) as u8;
    Ok(RGB { red: r, green: g, blue: b })
}

/// Checks for the 'success' key in order to determine whether an API response
/// indicates success or failure.
pub fn check_success(map: &collections::TreeMap<String, json::Json>) -> Result<(), error::Error> {
    let success_value = match (*map).find(&"success".to_string()) {
        Some(x) => x,
        None => return Err(
            error::Error::new(
                error::PlanetsNuError,
                "Could not find the 'success' key in the response.".to_string())),
    };
    let success = match *success_value {
        json::Boolean(x) => x,
        _ => return Err(
            error::Error::new(
                error::PlanetsNuError,
                "Unexpected (non-boolean) value found for 'success' key.".to_string())),
    };
    match success {
        true => Ok(()),
        false => {
            let unknown_err = error::Error::new(
                error::PlanetsNuError,
                "Response indicates that the request failed (reason unknown).".to_string());
            let error_value = match (*map).find(&"error".to_string()) {
                Some(x) => x,
                None => return Err(unknown_err),
            };
            match *error_value {
                json::String(ref x) => Err(
                    error::Error::new(
                        error::PlanetsNuError,
                        x.clone())),
                _ => return Err(unknown_err),
            }
        },
    }
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use super::serialize::json;
    use std::collections;
    use error;

    #[test]
    fn test_to_rgb_errors() {
        assert_eq!(error::LibError, to_rgb("#af320").unwrap_err().kind);   // too short
        assert_eq!(error::LibError, to_rgb("#af32021").unwrap_err().kind); // too long
        assert_eq!(error::LibError, to_rgb("#ag03a3").unwrap_err().kind);  // non-hex digit
        assert_eq!(error::LibError, to_rgb("1234567").unwrap_err().kind);  // no leading '#'
    }

    #[test]
    fn test_to_rgb() {
        assert_eq!(RGB { red: 175u8, green: 50u8, blue: 8u8 }, to_rgb("#af3208").unwrap());
    }

    #[test]
    fn test_check_success_errors() {
        let mut map = collections::TreeMap::new();
        assert_eq!(error::PlanetsNuError, check_success(&map).unwrap_err().kind); // no "success" key
        map.insert("success".to_string(), json::I64(1337i64));
        assert_eq!(error::PlanetsNuError, check_success(&map).unwrap_err().kind); // wrong type for "success" key
        map.insert("success".to_string(), json::Boolean(false));
        assert_eq!(error::PlanetsNuError, check_success(&map).unwrap_err().kind); // request failed, unknown reason
        map.insert("error".to_string(), json::String("heh".to_string()));
        assert_eq!(error::PlanetsNuError, check_success(&map).unwrap_err().kind); // request failed, known reason
    }

    #[test]
    fn test_check_success() {
        let mut map = collections::TreeMap::new();
        map.insert("success".to_string(), json::Boolean(true));
        assert_eq!((), check_success(&map).unwrap());
    }
}
