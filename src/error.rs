/*!
Error-related data structures and helpers.
*/

/// Specifies the kind for an error produced by the planets_nu library.
#[deriving(Eq, PartialEq, Show)]
pub enum ErrorKind {
    LibError,
    NetworkError,
    PlanetsNuError,
}

/// Represents an error returned by the planets_nu library.
#[deriving(Eq, PartialEq, Show)]
pub struct Error {
    pub kind: ErrorKind,
    pub desc: String,
}

impl Error {
    /// Constructs a new instance of an Error.
    pub fn new(kind: ErrorKind, desc: String) -> Error {
        Error { kind: kind, desc: desc }
    }
}
