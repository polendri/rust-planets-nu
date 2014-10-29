/*!
Contains data structures and helper methods useful to builder modules.
*/
extern crate serialize;

use self::serialize::json;
use std::collections;

use error;
use json_helpers::{find, get_i64};

// Public

pub fn map_with_err<T, R, E>(vec: &Vec<T>, f: |&T| -> Result<R, E>) -> Result<Vec<R>, E> {
    let mut result: Vec<R> = Vec::with_capacity(vec.len());
    for item in vec.iter() {
        match f(item) {
            Ok(x) => result.push(x),
            Err(e) => return Err(e),
        }
    }
    Ok(result)
}

/// Reads an (x,y) coordinate pair from a JSON object.
pub fn get_coordinates(obj: &json::Json) -> Result<(i64, i64), error::Error> {
    match *obj {
        json::Object(ref map) => Ok(
            (try!(get_i64(try!(find(map, "x")))), try!(get_i64(try!(find(map, "y")))))),
        _ => Err(error::Error::new(
            error::LibError,
            "Expected a JSON coordinate object but found something else.".to_string())),
    }
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
    fn test_get_coordinates() {
        assert_eq!((123i64, 4i64), get_coordinates(&json::from_str("{ \"x\": 123, \"y\": 4 }").unwrap()).unwrap());
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
