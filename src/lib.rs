#![feature(macro_rules)]

macro_rules! expect(
    ($e1:expr, $e2:expr) => (match $e1 { Some(s) => s, None => return Err($e2) })
)

macro_rules! try_match(
    ($e1:expr, $p:pat => $e2:expr, $e3:expr) => (match $e1 { $p => $e2, _ => return Err($e3) })
)

pub mod planets_nu;

mod curl;
