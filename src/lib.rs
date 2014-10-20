#![crate_name="planets_nu"]
#![crate_type="rlib"]

#![feature(macro_rules)]

macro_rules! expect(
    ($e1:expr, $e2:expr) => (match $e1 { Some(s) => s, None => return Err($e2) })
)

macro_rules! try_match(
    ($e1:expr, $p:pat => $e2:expr, $e3:expr) => (match $e1 { $p => $e2, _ => return Err($e3) })
)

macro_rules! find_match(
    ($i:ident, $e1:expr, $p:pat => $e2:expr) => (try_match!(*expect!($i.find(&$e1.to_string()), common::Error::new(common::LibError, format!("Could not find key '{}'.", $e1))), $p => $e2, common::Error::new(common::LibError, format!("Unexpected type for key '{}'.", $e1))))
)

macro_rules! match_json_object(
    ($i:ident, $e:expr) => (find_match!($i, $e, json::Object(ref x) => x))
)

macro_rules! match_json_bool(
    ($i:ident, $e:expr) => (find_match!($i, $e, json::Boolean(x) => x))
)

macro_rules! match_json_string(
    ($i:ident, $e:expr) => (find_match!($i, $e, json::String(ref x) => x.clone()))
)

pub mod request;

mod common;
mod curl;
mod login;
mod player_settings;
