/*!
Functions to make requests against the planets.nu API.
*/
extern crate curl;
extern crate flate2;

use self::curl::http;
use self::flate2::reader::GzDecoder;
use std::io::BufReader;
use std::str;
use std::vec;

use error;
use builders::game;
use builders::load_turn;
use builders::login;
use parse;

// Public

bitflags! {
    flags GameStatusFlags: u8 {
        const STATUS_DEFAULT  = 0x00,
        const STATUS_JOINING  = 0x01,
        const STATUS_RUNNING  = 0x02,
        const STATUS_FINISHED = 0x04,
        const STATUS_HOLD     = 0x08,
    }
}

bitflags! {
    flags GameTypeFlags: u8 {
        const GAME_TYPE_DEFAULT   = 0x00,
        const GAME_TYPE_TRAINING  = 0x01,
        const GAME_TYPE_STANDARD  = 0x02,
        const GAME_TYPE_TEAM      = 0x04,
        const GAME_TYPE_MELEE     = 0x08,
        const GAME_TYPE_BLITZ     = 0x10,
    }
}

pub enum GameScope {
    DefaultScope,
    PublicScope,
    CustomScope,
}

/// Make a call to the login API.
///
/// The purpose of this call is to retrieve an API key that can be passed along in other API
/// requests. It also retrieves the settings for the user.
pub fn login(username: &str, password: &str) -> Result<login::LoginResult, error::Error> {
    let url = format!("http://api.planets.nu/login?username={0}&password={1}", username, password);
    let response = try!(http_get(url.as_slice()));
    parse::login(try!(decode_response(&response)).as_slice())
}

/// Make a call to the games list API.
///
/// This call retrieves the list of all games, which can be filtered by several criteria.
///
/// status: Retrieve only games with the specified statuses.
/// game_type: Retrieve only games with the specified game types.
/// scope: Retrieve only games with the specified scope.
/// ids: Retrieve only games with the specified IDs.
/// username: Username of account to list games.
///
/// Will list all games for this user regardless of settings.
/// The maximum number of records to return.
// TODO: document the parameters better
pub fn list_games(
    status: GameStatusFlags,
    game_type: GameTypeFlags,
    scope: GameScope,
    ids: &Vec<i64>,
    username: Option<&str>,
    limit: Option<i64>) -> Result<Vec<game::Game>, error::Error>
{
    let url = build_games_list_url(status, game_type, scope, ids, username, limit);
    let response = try!(http_get(url.as_slice()));
    parse::list_games(try!(decode_response(&response)).as_slice())
}


/// Make a call to the load turn API.
///
/// This call retrieves all information relating to a single turn of a game.
/// game_id: The game ID for which the turn is requested.
/// turn: The turn number requested (if unspecified, the latest turn is used).
/// api_key: The API key from the login API call. Used to authenticate calls for active games.
/// player_id: The player to request a turn for.
///
/// In an active game this must be the ID of the player to which the API key belongs.
/// for_save: Indicates if the client intends to make a call to the save turn API.
// TODO: document the parameters better
pub fn load_turn(
    game_id: i64,
    turn: Option<i64>,
    api_key: Option<&str>,
    player_id: Option<i64>,
    for_save: bool) -> Result<load_turn::LoadTurnResult, error::Error>
{

    let url = build_load_turn_url(game_id, turn, api_key, player_id, for_save);
    let response = try!(http_get(url.as_slice()));
    parse::load_turn(try!(decode_response(&response)).as_slice())
}

// Private

fn status_flags_to_str(flags: GameStatusFlags) -> Option<String> {
    match flags.is_empty() {
        true => None,
        false => {
            let mut statuses = vec::Vec::new();
            if flags.contains(STATUS_JOINING) { statuses.push("1"); }
            if flags.contains(STATUS_RUNNING) { statuses.push("2"); }
            if flags.contains(STATUS_FINISHED) { statuses.push("3"); }
            if flags.contains(STATUS_HOLD) { statuses.push("4"); }
            Some(statuses.connect(","))
        },
    }
}

fn type_flags_to_str(flags: GameTypeFlags) -> Option<String> {
    match flags.is_empty() {
        true => None,
        false => {
            let mut types = vec::Vec::new();
            if flags.contains(GAME_TYPE_TRAINING) { types.push("1"); }
            if flags.contains(GAME_TYPE_STANDARD) { types.push("2"); }
            if flags.contains(GAME_TYPE_TEAM) { types.push("3"); }
            if flags.contains(GAME_TYPE_MELEE) { types.push("4"); }
            if flags.contains(GAME_TYPE_BLITZ) { types.push("5"); }
            Some(types.connect(","))
        },
    }
}

/// Builds the URL used for the games list API.
fn build_games_list_url(status: GameStatusFlags,
                        game_type: GameTypeFlags,
                        scope: GameScope,
                        ids: &Vec<i64>,
                        username: Option<&str>,
                        limit: Option<i64>) -> String {
    let mut url = "http://api.planets.nu/games/list".to_string();
    let mut prepend_char = "?".to_string();

    match status_flags_to_str(status) {
        Some(s) => {
            url = url + prepend_char + "status=" + s;
            prepend_char = "&".to_string();
        },
        None => (),
    };

    match type_flags_to_str(game_type) {
        Some(s) => {
            url = url + prepend_char + "type=" + s;
            prepend_char = "&".to_string();
        },
        None => (),
    };

    match scope {
        DefaultScope => (),
        PublicScope => {
            url = url + prepend_char + "scope=0";
            prepend_char = "&".to_string();
        },
        CustomScope => {
            url = url + prepend_char + "scope=1";
            prepend_char = "&".to_string();
        },
    };

    let ids_as_str: Vec<String> = ids.iter().map(|x| format!("{}", x)).collect();
    if ids_as_str.len() > 0 {
        url = url + prepend_char + "ids=" + ids_as_str.connect(",");
        prepend_char = "&".to_string();
    }

    match username {
        Some(s) => {
            url = url + prepend_char + "username=" + s;
            prepend_char = "&".to_string();
        },
        None => (),
    };

    match limit {
        Some(i) => {
            url = url + prepend_char + "limit=" + i.to_string();
        },
        None => (),
    };

    url
}

/// Builds the URL used for the load turn API.
fn build_load_turn_url(game_id: i64,
                       turn: Option<i64>,
                       api_key: Option<&str>,
                       player_id: Option<i64>,
                       for_save: bool) -> String {
    let mut url = "http://api.planets.nu/game/loadturn?gameid=".to_string() + game_id.to_string();

    match turn {
        Some(i) => url = url + "&turn=" + i.to_string(),
        None => (),
    };

    match api_key {
        Some(s) => url = url + "&apikey=" + s.to_string(),
        None => (),
    };

    match player_id {
        Some(i) => url = url + "&playerid=" + i.to_string(),
        None => (),
    };

    url + "&forsave=" + for_save.to_string()
}

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

/// Returns the body of the response, decoding it if it is compressed.
fn decode_response(response: &http::Response) -> Result<String, error::Error> {
    let enc_headers = response.get_header("content-encoding");
    if enc_headers.len() == 0 {
        return Ok(try!(bytes_to_str(response.get_body())).to_string());
    }
    match enc_headers[0].as_slice() {
        "gzip" => {
            let mut decoder = GzDecoder::new(BufReader::new(response.get_body()));
            match decoder.read_to_string() {
                Ok(s) => Ok(s.clone()),
                Err(error) => Err(error::Error::new(
                    error::NetworkError,
                    format!("Unable to decode the GZIP-compressed response: {}", error))),
            }
        },
        _ => Ok(try!(bytes_to_str(response.get_body())).to_string()),
    }
}

/// Converts a byte slice into a string, returning an error on failure.
fn bytes_to_str<'a>(bytes: &'a [u8]) -> Result<&'a str, error::Error> {
    match str::from_utf8(bytes) {
        Some(s) => Ok(s),
        None => Err(error::Error::new(
            error::NetworkError,
            "Response body is not valid UTF-8.".to_string())),
    }
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use super::build_games_list_url;
    use super::build_load_turn_url;
    use request;

    #[test]
    fn test_build_games_list_url_defaults() {
        assert_eq!(
            "http://api.planets.nu/games/list",
            build_games_list_url(
                request::STATUS_DEFAULT,
                request::GAME_TYPE_DEFAULT,
                request::DefaultScope,
                &Vec::new(),
                None,
                None).as_slice());
    }

    #[test]
    fn test_build_games_list_url() {
        assert_eq!(
            "http://api.planets.nu/games/list\
                ?status=1,3&type=1,4&scope=1&ids=12,13371337&username=theuser&limit=123",
            build_games_list_url(
                request::STATUS_JOINING | request::STATUS_FINISHED,
                request::GAME_TYPE_TRAINING | request::GAME_TYPE_MELEE,
                request::CustomScope,
                &vec![12,13371337],
                Some("theuser"),
                Some(123i64)).as_slice());
    }

    #[test]
    fn test_build_load_turn_url_defaults() {
        assert_eq!(
            "http://api.planets.nu/game/loadturn?gameid=1337&forsave=false",
            build_load_turn_url(1337, None, None, None, false).as_slice());
    }

    #[test]
    fn test_build_load_turn_url() {
        assert_eq!(
            "http://api.planets.nu/game/loadturn?gameid=1337&turn=5&apikey=theapikey&playerid=123&forsave=true",
            build_load_turn_url(1337, Some(5i64), Some("theapikey"), Some(123i64), true).as_slice());
    }
}
