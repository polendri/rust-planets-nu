/*!
Contains data structures and helper methods useful
to many components of the library.
*/

extern crate serialize;

use self::serialize::json;
use std::char;
use std::collections;

use error;

#[deriving(Eq, PartialEq, Show)]
pub struct RGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

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
    try_match!(iter.next().unwrap(), '#' => (), str_to_err("Could not find leading '#'.".to_string()));
    let r = (expect!(char::to_digit(iter.next().unwrap(), 16), digit_err) as u8 << 4) +
        expect!(char::to_digit(iter.next().unwrap(), 16), digit_err) as u8;
    let g = (expect!(char::to_digit(iter.next().unwrap(), 16), digit_err) as u8 << 4) +
        expect!(char::to_digit(iter.next().unwrap(), 16), digit_err) as u8;
    let b = (expect!(char::to_digit(iter.next().unwrap(), 16), digit_err) as u8 << 4) +
        expect!(char::to_digit(iter.next().unwrap(), 16), digit_err) as u8;
    Ok(RGB { red: r, green: g, blue: b })
}

/// Converts a string which encodes a JSON object into a TreeMap representing that object.
pub fn json_to_map(json: &str) -> Result<collections::TreeMap<String, json::Json>, error::Error> {
    let root_enum = match json::from_str(json) {
        Ok(x) => x,
        Err(error) => return Err(error::Error::new(error::PlanetsNuError, format!("Error while decoding the login response: {}", error))),
    };
    Ok(try_match!(root_enum, json::Object(x) => x, error::Error::new(error::PlanetsNuError, "Could not find root of login response".to_string())))
}

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
fn test_json_to_map() {
    let input = "{ \"key1\" : true, \"key2\" : \"value\" }";
    let map = json_to_map(input).unwrap();
    assert_eq!(json::Boolean(true), *map.find(&"key1".to_string()).unwrap());
    assert_eq!(json::String("value".to_string()), *map.find(&"key2".to_string()).unwrap());
}