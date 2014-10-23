#![crate_name="planets_nu"]
#![crate_type="rlib"]
#![doc(html_root_url = "http://www.rust-ci.org/pshendry/rust-planets-nu/doc/planets_nu/")]

#![feature(macro_rules)]

macro_rules! expect(
    ($e1:expr, $e2:expr) => (match $e1 { Some(s) => s, None => return Err($e2) })
)

pub mod error;
pub mod parse;
pub mod request;

mod builders;
mod json_helpers;
