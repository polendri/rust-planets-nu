extern crate serialize;

use self::serialize::json;

use error;
use json_helpers::{find, get_bool, get_i32, get_object, get_string};

// Public

#[deriving(Eq, PartialEq, Show)]
pub struct Player {
    pub status: i32,
    pub status_turn: i32,
    pub account_id: i32,
    pub username: String,
    pub email: String,
    pub race_id: i32,
    pub team_id: i32,
    pub priority_points: i32,
    pub join_rank: i32,
    pub finish_rank: i32,
    pub turn_joined: i32,
    pub turn_ready: bool,
    pub turn_ready_datetime: String,
    pub turn_status: i32,
    pub turns_missed: i32,
    pub turns_missed_total: i32,
    pub turns_holiday: i32,
    pub turns_early: i32,
    pub active_hulls: Vec<String>,
    pub active_advantages: Vec<String>,
    pub save_key: String,
    pub tutorial_id: i32,
    pub tutorial_task_id: i32,
    pub id: i32,
}

pub fn build(json: &json::Json) -> Result<Player, error::Error> {
    let map = try!(get_object(json));
    Ok(Player {
        status:
            try!(get_i32(try!(find(map, "status")))),
        status_turn:
            try!(get_i32(try!(find(map, "statusturn")))),
        account_id:
            try!(get_i32(try!(find(map, "accountid")))),
        username:
            try!(get_string(try!(find(map, "username")))),
        email:
            try!(get_string(try!(find(map, "email")))),
        race_id:
            try!(get_i32(try!(find(map, "raceid")))),
        team_id:
            try!(get_i32(try!(find(map, "teamid")))),
        priority_points:
            try!(get_i32(try!(find(map, "prioritypoints")))),
        join_rank:
            try!(get_i32(try!(find(map, "joinrank")))),
        finish_rank:
            try!(get_i32(try!(find(map, "finishrank")))),
        turn_joined:
            try!(get_i32(try!(find(map, "turnjoined")))),
        turn_ready:
            try!(get_bool(try!(find(map, "turnready")))),
        turn_ready_datetime:
            try!(get_string(try!(find(map, "turnreadydate")))),
        turn_status:
            try!(get_i32(try!(find(map, "turnstatus")))),
        turns_missed:
            try!(get_i32(try!(find(map, "turnsmissed")))),
        turns_missed_total:
            try!(get_i32(try!(find(map, "turnsmissedtotal")))),
        turns_holiday:
            try!(get_i32(try!(find(map, "turnsholiday")))),
        turns_early:
            try!(get_i32(try!(find(map, "turnsearly")))),
        active_hulls:
            try!(get_string(try!(find(map, "activehulls")))).as_slice().split(',').map(|x| x.to_string()).collect(),
        active_advantages:
            try!(get_string(try!(find(map, "activeadvantages")))).as_slice().split(',').map(|x| x.to_string()).collect(),
        save_key:
            try!(get_string(try!(find(map, "savekey")))),
        tutorial_id:
            try!(get_i32(try!(find(map, "tutorialid")))),
        tutorial_task_id:
            try!(get_i32(try!(find(map, "tutorialtaskid")))),
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
            status: 1i32,
            status_turn: 1i32,
            account_id: 12345i32,
            username: "that user name".to_string(),
            email: "".to_string(),
            race_id: 9i32,
            team_id: 0i32,
            priority_points: 0i32,
            join_rank: 0i32,
            finish_rank: 0i32,
            turn_joined: 1i32,
            turn_ready: true,
            turn_ready_datetime: "10/8/2014 5:47:45 PM".to_string(),
            turn_status: 2i32,
            turns_missed: 0i32,
            turns_missed_total: 0i32,
            turns_holiday: 0i32,
            turns_early: 5i32,
            active_hulls: vec!["15","16","17","18","78","79","80","81","82","83","84","85","104","105"].iter().map(|x| x.to_string()).collect(),
            active_advantages: vec!["24","25","45","48","49","51"].iter().map(|x| x.to_string()).collect(),
            save_key: "".to_string(),
            tutorial_id: 0i32,
            tutorial_task_id: 0i32,
            id: 3i32,
        };
        assert_eq!(result, build(&parse(json).unwrap()).unwrap());
    }
}
