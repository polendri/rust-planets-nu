extern crate serialize;

use self::serialize::json;

use error;
use json_helpers::{find, get_bool, get_i32, get_object, get_string};

macro_rules! get(
    ($i1:ident, $e:expr, $i2:ident) => (try!($i2(try!(find($i1, $e)))))
)

// Public

#[deriving(Eq, PartialEq, Show)]
pub struct Minefield {
    pub owner_id: i32,
    pub is_web: bool,
    pub units: i32,
    pub info_turn: i32,
    pub friendly_code: String,
    pub position: (i32, i32),
    pub radius: i32,
    pub id: i32,
}

pub fn build(json: &json::Json) -> Result<Minefield, error::Error> {
    let map = try!(get_object(json));
    Ok(Minefield {
        owner_id: get!(map, "ownerid", get_i32),
        is_web: get!(map, "isweb", get_bool),
        units: get!(map, "units", get_i32),
        info_turn: get!(map, "infoturn", get_i32),
        friendly_code: get!(map, "friendlycode", get_string),
        position: (get!(map, "x", get_i32), get!(map, "y", get_i32)),
        radius: get!(map, "radius", get_i32),
        id: get!(map, "id", get_i32),
    })
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use json_helpers::parse;

    #[test]
    fn test_build() {
        let json = "{\
            \"ownerid\":7,\
            \"isweb\":true,\
            \"units\":1253,\
            \"infoturn\":17,\
            \"friendlycode\":\"???\",\
            \"x\":2729,\
            \"y\":2335,\
            \"radius\":35,\
            \"id\":1\
        }";
        let result = Minefield {
            owner_id: 7i32,
            is_web: true,
            units: 1253i32,
            info_turn: 17i32,
            friendly_code: "???".to_string(),
            position: (2729i32, 2335i32),
            radius: 35i32,
            id: 1i32,
        };
        assert_eq!(result, build(&parse(json).unwrap()).unwrap());
    }
}
