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

    print_result("login", request::login("rusty314159", "rusty314159"));
    // TODO: games list test

    println!("\nOnline test run has completed.");
}
