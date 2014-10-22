/*!
Functions to make requests against the planets.nu API.
*/
extern crate curl;

use self::curl::http;
use std::str;

use error;
use builders::login;
pub use builders::login::LoginResult;
use parse;

/// Performs an HTTP GET request, returning the response (or an error).
fn http_get(url: &str) -> Result<http::Response, error::Error> {
    match http::handle().get(url).exec() {
        Ok(x) => Ok(x),
        Err(code) => Err(error::Error::new(
            error::NetworkError,
            format!("curl GET request failed with error code {}", code))),
    }
}

/*
/// Performs an HTTP POST request, returning the response (or an error).
fn http_post(url: &str, data: &str) -> Result<http::Response, error::Error> {
    match http::handle().post(url, data).exec() {
        Ok(x) => Ok(x),
        Err(code) => Err(error::Error::new(
            error::NetworkError,
            format!("curl POST request failed with error code {}", code))),
    }
}
*/

fn bytes_to_str<'a>(bytes: &'a [u8]) -> Result<&'a str, error::Error> {
    match str::from_utf8(bytes) {
        Some(s) => Ok(s),
        None => Err(error::Error::new(
            error::NetworkError,
            "Response body is not valid UTF-8.".to_string())),
    }
}

/// Make a call to the login API.
///
/// TODO: Way more documentation; code examples.
pub fn login(username: &str, password: &str) -> Result<login::LoginResult, error::Error> {
    let url = format!("http://api.planets.nu/login?username={0}&password={1}", username, password);
    let response = try!(http_get(url.as_slice()));
    parse::login(try!(bytes_to_str(response.get_body())))
}

/*
/// Make a call to the games list API.
///
/// TODO: Way more documentation; code examples.
/// TODO: params should have their own structs
pub fn list_games(status: Option<i64>,
                  game_type: Option<i64>,
                  scope: Option<i64>,
                  ids: Option<&str>,
                  username: Option<&str>,
                  limit: Option<i64>) -> Result<login::LoginResult, error::Error> {
    let mut url = "http://api.planets.nu/games/list".to_string();

    if status.is_some() || game_type.is_some() || scope.is_some() || ids.is_some() || username.is_some() || limit.is_some()
    ?username={0}&password={1}", username, password);
    let response = try!(http_get(url.as_slice()));
    parse::login(try!(bytes_to_str(response.get_body())))
}
*/

/*
pub fn game_info_json(game_id: i64) -> Result<String, String> {
    let url = "http://api.planets.nu/game/loadinfo?gameid=".to_string() + game_id.to_string();
*/
