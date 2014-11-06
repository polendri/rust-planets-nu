extern crate serialize;

use self::serialize::json;

use error;
use json_helpers::{find, get_float, get_i32, get_object, get_string};

// Public

#[deriving(Eq, PartialEq, Show)]
pub struct PlayerScore {
    pub added_datetime: String,
    pub owner_id: i32,
    pub account_id: i32,
    pub capital_ships: i32,
    pub freighters: i32,
    pub planets: i32,
    pub starbases: i32,
    pub military_score: i32,
    pub inventory_score: i32,
    pub priority_points: i32,
    pub turn: i32,
    pub percent: String,
    pub id: i32,
}

pub fn build(json: &json::Json) -> Result<PlayerScore, error::Error> {
    let map = try!(get_object(json));
    Ok(PlayerScore {
        added_datetime:
            try!(get_string(try!(find(map, "dateadded")))),
        owner_id:
            try!(get_i32(try!(find(map, "ownerid")))),
        account_id:
            try!(get_i32(try!(find(map, "accountid")))),
        capital_ships:
            try!(get_i32(try!(find(map, "capitalships")))),
        freighters:
            try!(get_i32(try!(find(map, "freighters")))),
        planets:
            try!(get_i32(try!(find(map, "planets")))),
        starbases:
            try!(get_i32(try!(find(map, "starbases")))),
        military_score:
            try!(get_i32(try!(find(map, "militaryscore")))),
        inventory_score:
            try!(get_i32(try!(find(map, "inventoryscore")))),
        priority_points:
            try!(get_i32(try!(find(map, "prioritypoints")))),
        turn:
            try!(get_i32(try!(find(map, "turn")))),
        percent:
            try!(get_float(try!(find(map, "percent")), 2u)),
        id:
            try!(get_i32(try!(find(map, "id")))),
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
            \"dateadded\": \"10/8/2014 5:10:54 PM\",\
            \"ownerid\": 1,\
            \"accountid\": 12345,\
            \"capitalships\": 5,\
            \"freighters\": 2,\
            \"planets\": 11,\
            \"starbases\": 1,\
            \"militaryscore\": 10754,\
            \"inventoryscore\": 282,\
            \"prioritypoints\": 0,\
            \"turn\": 7,\
            \"percent\": 17.88,\
            \"id\": 37,\
            \"shipchange\": 1,\
            \"freighterchange\": 0,\
            \"planetchange\": 4,\
            \"starbasechange\": 0,\
            \"militarychange\": 1099,\
            \"inventorychange\": 50,\
            \"prioritypointchange\": 0,\
            \"percentchange\": -0.73\
        }";
        let result = PlayerScore {
            added_datetime: "10/8/2014 5:10:54 PM".to_string(),
            owner_id: 1i32,
            account_id: 12345i32,
            capital_ships: 5i32,
            freighters: 2i32,
            planets: 11i32,
            starbases: 1i32,
            military_score: 10754i32,
            inventory_score: 282i32,
            priority_points: 0i32,
            turn: 7i32,
            percent: "17.88".to_string(),
            id: 37i32,
        };
        assert_eq!(result, build(&parse(json).unwrap()).unwrap());
    }
}
