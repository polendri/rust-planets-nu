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

pub fn parse(json: &str) -> Result<json::Json, error::Error> {
    match json::from_str(json) {
        Ok(x) => Ok(x),
        Err(error) => Err(error::Error::new(
            error::PlanetsNuError,
            format!("Error while decoding JSON: {}", error))),
    }
}

/// Find a key in the map, or else return an error.
pub fn find<'a>(map: &'a collections::TreeMap<String, json::Json>, key: &str) -> Result<&'a json::Json, error::Error> {
    match (*map).find(&key.to_string()) {
        Some(x) => Ok(x),
        None => mk_lib_err(format!("Could not find key '{}'.", key)),
    }
}

pub fn get_object<'a>(json_enum: &'a json::Json) -> Result<&'a collections::TreeMap<String, json::Json>, error::Error> {
    match *json_enum {
        json::Object(ref x) => Ok(x),
        _ => mk_lib_err("Expected object but found something else.".to_string()),
    }
}

pub fn get_list<'a>(json_enum: &'a json::Json) -> Result<&'a Vec<json::Json>, error::Error> {
    match *json_enum {
        json::List(ref x) => Ok(x),
        _ => mk_lib_err("Expected list but found something else.".to_string()),
    }
}

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

pub fn get_i64<'a>(json_enum: &'a json::Json) -> Result<i64, error::Error> {
    match *json_enum {
        json::I64(x) => Ok(x),
        json::U64(x) => Ok(x as i64),
        _ => mk_lib_err("Expected int but found something else.".to_string()),
    }
}

pub fn get_float<'a>(json_enum: &'a json::Json) -> Result<String, error::Error> {
    match *json_enum {
        json::F64(x) => Ok(f64::to_str_digits(x, 15)),
        _ => mk_lib_err("Expected float but found something else.".to_string()),
    }
}

pub fn get_string<'a>(json_enum: &'a json::Json) -> Result<String, error::Error> {
    match *json_enum {
        json::String(ref x) => Ok(x.clone()),
        _ => mk_lib_err("Expected string but found something else.".to_string()),
    }
}

// TODO: tests
/*
#[test]
fn test_json_to_map() {
    let input = "{ \"key1\" : true, \"key2\" : \"value\" }";
    let map = json_to_map(input).unwrap();
    assert_eq!(json::Boolean(true), *map.find(&"key1".to_string()).unwrap());
    assert_eq!(json::String("value".to_string()), *map.find(&"key2".to_string()).unwrap());
}
*/
