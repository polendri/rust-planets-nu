extern crate serialize;

use self::serialize::json;
use std::collections;

use common;
use curl;
use login;
pub use login::LoginResult;

fn check_success(map: &collections::TreeMap<String, json::Json>) -> Option<common::Error> {
    let success_value = match (*map).find(&"success".to_string()) {
        Some(x) => x,
        None => return Some(common::Error::new(common::PlanetsNuError, "Could not find the 'success' key in the response.".to_string())),
    };
    let success = match *success_value {
        json::Boolean(x) => x,
        _ => return Some(common::Error::new(common::PlanetsNuError, "Unexpected (non-boolean) value found for 'success' key.".to_string())),
    };
    match success {
        true => None,
        false => {
            let unknown_err = common::Error::new(common::PlanetsNuError, "Response indicates that the request failed (reason unknown).".to_string());
            let error_value = match (*map).find(&"error".to_string()) {
                Some(x) => x,
                None => return Some(unknown_err),
            };
            match *error_value {
                json::String(ref x) => Some(common::Error::new(common::PlanetsNuError, x.clone())),
                _ => return Some(unknown_err),
            }
        },
    }
}

pub fn login(username: String, password: String) -> Result<login::LoginResult, common::Error> {
    let url = "http://api.planets.nu/login";
    //let url = "http://api.planets.nu/login?username=rainbow%20stalin&password=qmb1337acusphail";
    let data = format!("username={0}&password={1}", username, password);
    let mut reader = try!(curl::http_post(url, data.as_slice()).map_err(|x| common::Error::new(common::NetworkError, x.desc.to_string())));
    let json = try!(reader.read_to_string().map_err(|x| common::Error::new(common::NetworkError, x.desc.to_string())));
    let map = try!(common::json_to_map(json.as_slice()));
    match check_success(&map) {
        Some(err) => return Err(err),
        None => (),
    };
    login::read(&try!(common::json_to_map(json.as_slice())))
}

/*
pub fn game_info_json(game_id: i64) -> Result<String, String> {
    let url = "http://api.planets.nu/game/loadinfo?gameid=".to_string() + game_id.to_string();
*/
