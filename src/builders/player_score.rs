extern crate serialize;

use self::serialize::json;

use error;
use json_helpers::{find, get_float, get_i64, get_object, get_string};

// Public

#[deriving(Eq, PartialEq, Show)]
pub struct PlayerScore {
    pub added_datetime: String,
    pub owner_id: i64,
    pub account_id: i64,
    pub capital_ships: i64,
    pub freighters: i64,
    pub planets: i64,
    pub starbases: i64,
    pub military_score: i64,
    pub inventory_score: i64,
    pub priority_points: i64,
    pub turn: i64,
    pub percent: String,
    pub id: i64,
}

pub fn build(json: &json::Json) -> Result<PlayerScore, error::Error> {
    let map = try!(get_object(json));
    Ok(PlayerScore {
        added_datetime:
            try!(get_string(try!(find(map, "dateadded")))),
        owner_id:
            try!(get_i64(try!(find(map, "ownerid")))),
        account_id:
            try!(get_i64(try!(find(map, "accountid")))),
        capital_ships:
            try!(get_i64(try!(find(map, "capitalships")))),
        freighters:
            try!(get_i64(try!(find(map, "freighters")))),
        planets:
            try!(get_i64(try!(find(map, "planets")))),
        starbases:
            try!(get_i64(try!(find(map, "starbases")))),
        military_score:
            try!(get_i64(try!(find(map, "militaryscore")))),
        inventory_score:
            try!(get_i64(try!(find(map, "inventoryscore")))),
        priority_points:
            try!(get_i64(try!(find(map, "prioritypoints")))),
        turn:
            try!(get_i64(try!(find(map, "turn")))),
        percent:
            try!(get_float(try!(find(map, "percent")), 2u)),
        id:
            try!(get_i64(try!(find(map, "id")))),
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
            owner_id: 1i64,
            account_id: 12345i64,
            capital_ships: 5i64,
            freighters: 2i64,
            planets: 11i64,
            starbases: 1i64,
            military_score: 10754i64,
            inventory_score: 282i64,
            priority_points: 0i64,
            turn: 7i64,
            percent: "17.88".to_string(),
            id: 37i64,
        };
        assert_eq!(result, build(&parse(json).unwrap()).unwrap());
    }
}
