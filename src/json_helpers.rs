/*!
Some helper functions for reading JSON values.
*/

extern crate serialize;

use self::serialize::json;
use std::collections;

use error;

/// Find a key in the map, or else return an error.
fn find_key<'a>(map: &'a collections::TreeMap<String, json::Json>, key: String) -> Result<&'a json::Json, error::Error> {
    match (*map).find(&key) {
        Some(x) => Ok(x),
        None => Err(error::Error::new(
            error::LibError,
            format!("Could not find key '{}'.", key))),
    }
}

pub fn find_bool(map: &collections::TreeMap<String, json::Json>, key: &str) -> Result<bool, error::Error> {
    match *try!(find_key(map, key.to_string())) {
        json::Boolean(x) => Ok(x),
        json::String(ref x) => match (*x).as_slice() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(error::Error::new(
                error::LibError,
                format!("Expected bool value for key '{}' but found a string.", key))),
        },
        _ => Err(error::Error::new(
            error::LibError,
            format!("Expected string value for key '{}' but found something else.", key))),
    }
}

pub fn find_i64(map: &collections::TreeMap<String, json::Json>, key: &str) -> Result<i64, error::Error> {
    match *try!(find_key(map, key.to_string())) {
        json::I64(x) => Ok(x),
        json::U64(x) => Ok(x as i64),
        _ => Err(error::Error::new(
            error::LibError,
            format!("Expected int type for key '{}' but found something else.", key))),
    }
}

pub fn find_string(map: &collections::TreeMap<String, json::Json>, key: &str) -> Result<String, error::Error> {
    match *try!(find_key(map, key.to_string())) {
        json::String(ref x) => Ok(x.clone()),
        _ => Err(error::Error::new(
            error::LibError,
            format!("Expected string type for key '{}' but found something else.", key))),
    }
}
