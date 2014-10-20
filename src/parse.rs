/*!
Functions used to interpret planets.nu API responses directly from JSON.
*/
extern crate serialize;

use self::serialize::json;
use std::collections;

use common;
use error;
use builders::login;
pub use builders::login::LoginResult;

/// Checks for the 'success' key in order to determine whether an API response
/// indicates success or failure.
fn check_success(map: &collections::TreeMap<String, json::Json>) -> Option<error::Error> {
    let success_value = match (*map).find(&"success".to_string()) {
        Some(x) => x,
        None => return Some(
            error::Error::new(
                error::PlanetsNuError,
                "Could not find the 'success' key in the response.".to_string())),
    };
    let success = match *success_value {
        json::Boolean(x) => x,
        _ => return Some(
            error::Error::new(
                error::PlanetsNuError,
                "Unexpected (non-boolean) value found for 'success' key.".to_string())),
    };
    match success {
        true => None,
        false => {
            let unknown_err = error::Error::new(
                error::PlanetsNuError,
                "Response indicates that the request failed (reason unknown).".to_string());
            let error_value = match (*map).find(&"error".to_string()) {
                Some(x) => x,
                None => return Some(unknown_err),
            };
            match *error_value {
                json::String(ref x) => Some(
                    error::Error::new(
                        error::PlanetsNuError,
                        x.clone())),
                _ => return Some(unknown_err),
            }
        },
    }
}

/// Parse a login API response.
///
/// TODO: Way more documentation; code examples.
pub fn login(json: &str) -> Result<login::LoginResult, error::Error> {
    let map = try!(common::json_to_map(json));
    match check_success(&map) {
        Some(err) => return Err(err),
        None => (),
    };
    login::build(&map)
}
