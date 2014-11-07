/*!
Runs tests against the actual planets.nu website to confirm that they return successfully.
*/
extern crate planets_nu;

use self::planets_nu::error;
use self::planets_nu::request;
use std::fmt;

/// Given a request, prints out a message indicating its success or failure.
#[allow(dead_code)]
fn print_result<T: fmt::Show>(name: &str, result: Result<T, error::Error>) {
    print!("Running test for '{}' call... ", name);
    match result {
        Ok(_) => println!("...Success"),
        Err(err) => println!("...Failed ({0}: {1})", err.kind, err.desc),
    };
}

#[allow(dead_code)]
fn main() {
    println!("Online test run has begun.\n");

    print_result("login", request::login("rusty314159".to_string(), "rusty314159".to_string()));
    print_result(
        "list_games",
        request::list_games(
            request::STATUS_JOINING | request::STATUS_RUNNING,
            request::GAME_TYPE_STANDARD | request::GAME_TYPE_TEAM | request::GAME_TYPE_MELEE,
            request::PublicScope,
            &Vec::new(),
            None,
            Some(0i32)));
    print_result("load_turn", request::load_turn(815i32, None, None, Some(1i32), false));

    println!("\nOnline test run has completed.");
}
