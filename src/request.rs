/*!
Functions to make requests against the planets.nu API.
*/
use curl;
use error;
use builders::login;
pub use builders::login::LoginResult;
use parse;

/// Performs an HTTP GET or POST as appropriate, and returns
/// the resulting JSON (or an error if it fails).
fn get_json(url: &str, data: Option<String>) -> Result<String, error::Error> {
    let result = match data {
        Some(data) => curl::http_post(url, data.as_slice()),
        None => curl::http_get(url),
    };
    let mut reader = try!(result.map_err(
        |x| error::Error::new(error::NetworkError, x.desc.to_string())));
    reader.read_to_string().map_err(
        |x| error::Error::new(error::NetworkError, x.desc.to_string()))
}

/// Make a call to the login API.
///
/// TODO: Way more documentation; code examples.
pub fn login(username: &str, password: &str) -> Result<login::LoginResult, error::Error> {
    let url = "http://api.planets.nu/login";
    let data = format!("username={0}&password={1}", username, password);
    let json = try!(get_json(url, Some(data)));
    parse::login(json.as_slice())
}

/*
pub fn game_info_json(game_id: i64) -> Result<String, String> {
    let url = "http://api.planets.nu/game/loadinfo?gameid=".to_string() + game_id.to_string();
*/
