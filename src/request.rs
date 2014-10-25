/*!
Functions to make requests against the planets.nu API.
*/
extern crate curl;

use self::curl::http;
use std::str;

use error;
use builders::game;
use builders::login;
use parse;

// Public

/// Make a call to the login API.
///
/// The purpose of this call is to retrieve an API key that can be passed along in other API
/// requests. It also retrieves the settings for the user.
pub fn login(username: &str, password: &str) -> Result<login::LoginResult, error::Error> {
    let url = format!("http://api.planets.nu/login?username={0}&password={1}", username, password);
    let response = try!(http_get(url.as_slice()));
    parse::login(try!(bytes_to_str(response.get_body())))
}

/// Make a call to the games list API.
///
/// This call retrieves the list of all games, which can be filtered by several criteria.
pub fn list_games(status: Option<&str>,
                  game_type: Option<&str>,
                  scope: Option<&str>,
                  ids: Option<&str>,
                  username: Option<&str>,
                  limit: Option<i64>) -> Result<Vec<game::Game>, error::Error> {
    let mut url = "http://api.planets.nu/games/list".to_string();
    let mut prepend_char = "?".to_string();

    match status {
        Some(s) => {
            url = url + prepend_char + "status=" + s;
            prepend_char = "&".to_string();
        },
        None => (),
    };

    match game_type {
        Some(s) => {
            url = url + prepend_char + "type=" + s;
            prepend_char = "&".to_string();
        },
        None => (),
    };

    match scope {
        Some(s) => {
            url = url + prepend_char + "scope=" + s;
            prepend_char = "&".to_string();
        },
        None => (),
    };

    match ids {
        Some(s) => {
            url = url + prepend_char + "ids=" + s;
            prepend_char = "&".to_string();
        },
        None => (),
    };

    match username {
        Some(s) => {
            url = url + prepend_char + "username=" + s;
            prepend_char = "&".to_string();
        },
        None => (),
    };

    match limit {
        Some(i) => {
            url = url + prepend_char + "status=" + i.to_string();
        },
        None => (),
    };

    let response = try!(http_get(url.as_slice()));
    parse::list_games(try!(bytes_to_str(response.get_body())))
}

// Private

/// Performs an HTTP GET request, returning the response (or an error).
fn http_get(url: &str) -> Result<http::Response, error::Error> {
    match http::handle().get(url).exec() {
        Ok(x) => Ok(x),
        Err(code) => Err(error::Error::new(
            error::NetworkError,
            format!("curl GET request failed with error code {}", code))),
    }
}

// This works, but currently there is no need for POSTs.
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

/// Converts a byte slice into a string, returning an error on failure.
fn bytes_to_str<'a>(bytes: &'a [u8]) -> Result<&'a str, error::Error> {
    match str::from_utf8(bytes) {
        Some(s) => Ok(s),
        None => Err(error::Error::new(
            error::NetworkError,
            "Response body is not valid UTF-8.".to_string())),
    }
}
