/*!
Functions used to interpret planets.nu API responses directly from JSON.
*/
use builders::game;
use builders::list_games;
use builders::load_turn;
use builders::login;
use error;
use json_helpers::parse;

/// Parse a login API response.
pub fn login(json: &str) -> Result<login::LoginResult, error::Error> {
    login::build(&try!(parse(json)))
}

/// Parse a games list API response.
pub fn list_games(json: &str) -> Result<Vec<game::Game>, error::Error> {
    list_games::build(&try!(parse(json)))
}

/// Parse a load turn API response.
pub fn load_turn(json: &str) -> Result<load_turn::LoadTurnResult, error::Error> {
    load_turn::build(&try!(parse(json)))
}
