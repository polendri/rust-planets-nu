extern crate serialize;

use self::serialize::json;

use error;
use json_helpers::{find, get_bool, get_i64, get_object, get_string};

// Public

#[deriving(Eq, PartialEq, Show)]
pub struct Player {
    pub status: i64,
    pub status_turn: i64,
    pub account_id: i64,
    pub username: String,
    pub email: String,
    pub race_id: i64,
    pub team_id: i64,
    pub priority_points: i64,
    pub join_rank: i64,
    pub finish_rank: i64,
    pub turn_joined: i64,
    pub turn_ready: bool,
    pub turn_ready_datetime: String,
    pub turn_status: i64,
    pub turns_missed: i64,
    pub turns_missed_total: i64,
    pub turns_holiday: i64,
    pub turns_early: i64,
    pub active_hulls: Vec<String>,
    pub active_advantages: Vec<String>,
    pub save_key: String,
    pub tutorial_id: i64,
    pub tutorial_task_id: i64,
    pub id: i64,
}

pub fn build(json: &json::Json) -> Result<Player, error::Error> {
    let map = try!(get_object(json));
    Ok(Player {
        status:
            try!(get_i64(try!(find(map, "status")))),
        status_turn:
            try!(get_i64(try!(find(map, "statusturn")))),
        account_id:
            try!(get_i64(try!(find(map, "accountid")))),
        username:
            try!(get_string(try!(find(map, "username")))),
        email:
            try!(get_string(try!(find(map, "email")))),
        race_id:
            try!(get_i64(try!(find(map, "raceid")))),
        team_id:
            try!(get_i64(try!(find(map, "teamid")))),
        priority_points:
            try!(get_i64(try!(find(map, "prioritypoints")))),
        join_rank:
            try!(get_i64(try!(find(map, "joinrank")))),
        finish_rank:
            try!(get_i64(try!(find(map, "finishrank")))),
        turn_joined:
            try!(get_i64(try!(find(map, "turnjoined")))),
        turn_ready:
            try!(get_bool(try!(find(map, "turnready")))),
        turn_ready_datetime:
            try!(get_string(try!(find(map, "turnreadydate")))),
        turn_status:
            try!(get_i64(try!(find(map, "turnstatus")))),
        turns_missed:
            try!(get_i64(try!(find(map, "turnsmissed")))),
        turns_missed_total:
            try!(get_i64(try!(find(map, "turnsmissedtotal")))),
        turns_holiday:
            try!(get_i64(try!(find(map, "turnsholiday")))),
        turns_early:
            try!(get_i64(try!(find(map, "turnsearly")))),
        active_hulls:
            try!(get_string(try!(find(map, "activehulls")))).as_slice().split(',').map(|x| x.to_string()).collect(),
        active_advantages:
            try!(get_string(try!(find(map, "activeadvantages")))).as_slice().split(',').map(|x| x.to_string()).collect(),
        save_key:
            try!(get_string(try!(find(map, "savekey")))),
        tutorial_id:
            try!(get_i64(try!(find(map, "tutorialid")))),
        tutorial_task_id:
            try!(get_i64(try!(find(map, "tutorialtaskid")))),
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
            \"status\": 1,\
            \"statusturn\": 1,\
            \"accountid\": 12345,\
            \"username\": \"that user name\",\
            \"email\": \"\",\
            \"raceid\": 9,\
            \"teamid\": 0,\
            \"prioritypoints\": 0,\
            \"joinrank\": 0,\
            \"finishrank\": 0,\
            \"turnjoined\": 1,\
            \"turnready\": true,\
            \"turnreadydate\": \"10/8/2014 5:47:45 PM\",\
            \"turnstatus\": 2,\
            \"turnsmissed\": 0,\
            \"turnsmissedtotal\": 0,\
            \"turnsholiday\": 0,\
            \"turnsearly\": 5,\
            \"activehulls\": \"15,16,17,18,78,79,80,81,82,83,84,85,104,105\",\
            \"activeadvantages\": \"24,25,45,48,49,51\",\
            \"savekey\": \"\",\
            \"tutorialid\": 0,\
            \"tutorialtaskid\": 0,\
            \"id\": 3\
        }";
        let result = Player {
            status: 1i64,
            status_turn: 1i64,
            account_id: 12345i64,
            username: "that user name".to_string(),
            email: "".to_string(),
            race_id: 9i64,
            team_id: 0i64,
            priority_points: 0i64,
            join_rank: 0i64,
            finish_rank: 0i64,
            turn_joined: 1i64,
            turn_ready: true,
            turn_ready_datetime: "10/8/2014 5:47:45 PM".to_string(),
            turn_status: 2i64,
            turns_missed: 0i64,
            turns_missed_total: 0i64,
            turns_holiday: 0i64,
            turns_early: 5i64,
            active_hulls: vec!["15","16","17","18","78","79","80","81","82","83","84","85","104","105"].iter().map(|x| x.to_string()).collect(),
            active_advantages: vec!["24","25","45","48","49","51"].iter().map(|x| x.to_string()).collect(),
            save_key: "".to_string(),
            tutorial_id: 0i64,
            tutorial_task_id: 0i64,
            id: 3i64,
        };
        assert_eq!(result, build(&parse(json).unwrap()).unwrap());
    }
}
