/*!
Some helper functions for reading JSON values.
*/

extern crate serialize;

use self::serialize::json;
use std::collections;
use std::f64;

use error;

fn mk_lib_err<T>(msg: String) -> Result<T, error::Error> {
    Err(error::Error::new(error::LibError, msg))
}

/// Parses a JSON string and returns the root item, or else an error.
pub fn parse(json: &str) -> Result<json::Json, error::Error> {
    match json::from_str(json) {
        Ok(x) => Ok(x),
        Err(error) => Err(error::Error::new(
            error::PlanetsNuError,
            format!("Error while decoding JSON: {}", error))),
    }
}

/// Finds a key in the map, or else returns an error.
pub fn find<'a>(map: &'a collections::TreeMap<String, json::Json>, key: &str) -> Result<&'a json::Json, error::Error> {
    match (*map).find(&key.to_string()) {
        Some(x) => Ok(x),
        None => mk_lib_err(format!("Could not find key '{}'.", key)),
    }
}

/// Gets the value of a JSON object, returning an error if it is not an object.
pub fn get_object<'a>(json_enum: &'a json::Json) -> Result<&'a collections::TreeMap<String, json::Json>, error::Error> {
    match *json_enum {
        json::Object(ref x) => Ok(x),
        _ => mk_lib_err("Expected object but found something else.".to_string()),
    }
}

/// Gets the value of a JSON list, returning an error if it is not a list.
pub fn get_list<'a>(json_enum: &'a json::Json) -> Result<&'a Vec<json::Json>, error::Error> {
    match *json_enum {
        json::List(ref x) => Ok(x),
        _ => mk_lib_err("Expected list but found something else.".to_string()),
    }
}

/// Gets the value of a JSON boolean, returning an error if it is not a boolean.
pub fn get_bool<'a>(json_enum: &'a json::Json) -> Result<bool, error::Error> {
    match *json_enum {
        json::Boolean(x) => Ok(x),
        json::String(ref x) => match (*x).as_slice() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => mk_lib_err("Expected bool but found a string.".to_string()),
        },
        _ => mk_lib_err("Expected bool but found something else.".to_string()),
    }
}

/// Gets the value of a JSON integer, returning an error if it is not an integer.
pub fn get_i64<'a>(json_enum: &'a json::Json) -> Result<i64, error::Error> {
    match *json_enum {
        json::I64(x) => Ok(x),
        json::U64(x) => Ok(x as i64),
        _ => mk_lib_err("Expected int but found something else.".to_string()),
    }
}

/// Gets the value of a JSON float, returning an error if it is not a float.
///
/// Note: returns the result as a String to ensure exact representation,
/// because Rust does not appear to have a decimal type that is easy to use.
pub fn get_float<'a>(json_enum: &'a json::Json) -> Result<String, error::Error> {
    match *json_enum {
        json::F64(x) => Ok(f64::to_str_digits(x, 15)),
        _ => mk_lib_err("Expected float but found something else.".to_string()),
    }
}

/// Gets the value of a JSON string, returning an error if it is not a string.
pub fn get_string<'a>(json_enum: &'a json::Json) -> Result<String, error::Error> {
    match *json_enum {
        json::String(ref x) => Ok(x.clone()),
        _ => mk_lib_err("Expected string but found something else.".to_string()),
    }
}
