extern crate serialize;

use self::serialize::json;

use error;
use json_helpers::{find, get_bool, get_i64, get_object, get_string};

macro_rules! get(
    ($i1:ident, $e:expr, $i2:ident) => (try!($i2(try!(find($i1, $e)))))
)

// Public

#[deriving(Eq, PartialEq, Show)]
pub struct Minefield {
    pub owner_id: i64,
    pub is_web: bool,
    pub units: i64,
    pub info_turn: i64,
    pub friendly_code: String,
    pub position: (i64, i64),
    pub radius: i64,
    pub id: i64,
}

pub fn build(json: &json::Json) -> Result<Minefield, error::Error> {
    let map = try!(get_object(json));
    Ok(Minefield {
        owner_id: get!(map, "ownerid", get_i64),
        is_web: get!(map, "isweb", get_bool),
        units: get!(map, "units", get_i64),
        info_turn: get!(map, "infoturn", get_i64),
        friendly_code: get!(map, "friendlycode", get_string),
        position: (get!(map, "x", get_i64), get!(map, "y", get_i64)),
        radius: get!(map, "radius", get_i64),
        id: get!(map, "id", get_i64),
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
            owner_id: 7i64,
            is_web: true,
            units: 1253i64,
            info_turn: 17i64,
            friendly_code: "???".to_string(),
            position: (2729i64, 2335i64),
            radius: 35i64,
            id: 1i64,
        };
        assert_eq!(result, build(&parse(json).unwrap()).unwrap());
    }
}
