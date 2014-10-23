/*!
Functions used to interpret planets.nu API responses directly from JSON.
*/
use std::collections;

use builders::game;
use builders::list_games;
use builders::login;
pub use builders::login::LoginResult;
use error;
use json_helpers::parse;

/// Parse a login API response.
///
/// TODO: Way more documentation; code examples.
pub fn login(json: &str) -> Result<login::LoginResult, error::Error> {
    login::build(&try!(parse(json)))
}

/// Parse a games list API response.
///
/// TODO: Way more documentation; code examples.
pub fn list_games(json: &str) -> Result<Vec<game::Game>, error::Error> {
    list_games::build(&try!(parse(json)))
}
