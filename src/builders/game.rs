extern crate serialize;

use self::serialize::json;
use std::collections;

use common;
use error;
use json_helpers;

#[deriving(Eq, PartialEq, Show)]
pub struct Game {
    pub name: String,
    pub description: String,
    pub short_description: String,
    pub status: i64,
    pub created_datetime: String,
    pub ended_datetime: String,
    pub map_type: i64,
    pub game_type: i64,
    pub win_condition: i64,
    pub difficulty: String,
    pub tutorial_id: i64,
    pub required_level_id: i64,
    pub max_level_id: i64,
    pub master_planet_id: i64,
    pub quadrant: i64,
    pub min_tenacity: i64,
    pub fast_start: i64,
    pub turns_per_week: i64,
    pub year_started: i64,
    pub is_private: bool,
    pub scenario_id: i64,
    pub created_by: String,
    pub turn: i64,
    pub slots: i64,
    pub turn_status: String,
    pub host_days: String,
    pub slow_host_days: String,
    pub host_time: String,
    pub last_backup_path: String,
    pub next_host_datetime: String,
    pub all_turns_in: bool,
    pub last_notified: bool,
    pub is_hosting: bool,
    pub last_loaded_datetime: String,
    pub deleted_datetime: String,
    pub last_host_datetime: String,
    pub password: String,
    pub has_password: bool,
    pub status_name: String,
    pub just_ended: bool,
    pub time_to_host: String,
    pub id: i64,
}

/// Builds a game struct given a JSON object map.
pub fn build(map: &collections::TreeMap<String, json::Json>) -> Result<Game, error::Error> {
    Ok(Game {
        name:
            try!(json_helpers::find_string(map, "name")),
        description:
            try!(json_helpers::find_string(map, "description")),
        short_description:
            try!(json_helpers::find_string(map, "shortdescription")),
        status:
            try!(json_helpers::find_i64(map, "status")),
        created_datetime:
            try!(json_helpers::find_string(map, "datecreated")),
        ended_datetime:
            try!(json_helpers::find_string(map, "dateended")),
        map_type:
            try!(json_helpers::find_i64(map, "maptype")),
        game_type:
            try!(json_helpers::find_i64(map, "gametype")),
        win_condition:
            try!(json_helpers::find_i64(map, "wincondition")),
        difficulty:
            try!(json_helpers::find_float(map, "difficulty")),
        tutorial_id:
            try!(json_helpers::find_i64(map, "tutorialid")),
        required_level_id:
            try!(json_helpers::find_i64(map, "requiredlevelid")),
        max_level_id:
            try!(json_helpers::find_i64(map, "maxlevelid")),
        master_planet_id:
            try!(json_helpers::find_i64(map, "masterplanetid")),
        quadrant:
            try!(json_helpers::find_i64(map, "quadrant")),
        min_tenacity:
            try!(json_helpers::find_i64(map, "mintenacity")),
        fast_start:
            try!(json_helpers::find_i64(map, "faststart")),
        turns_per_week:
            try!(json_helpers::find_i64(map, "turnsperweek")),
        year_started:
            try!(json_helpers::find_i64(map, "yearstarted")),
        is_private:
            try!(json_helpers::find_bool(map, "isprivate")),
        scenario_id:
            try!(json_helpers::find_i64(map, "scenarioid")),
        created_by:
            try!(json_helpers::find_string(map, "createdby")),
        turn:
            try!(json_helpers::find_i64(map, "turn")),
        slots:
            try!(json_helpers::find_i64(map, "slots")),
        turn_status:
            try!(json_helpers::find_string(map, "turnstatus")),
        host_days:
            try!(json_helpers::find_string(map, "hostdays")),
        slow_host_days:
            try!(json_helpers::find_string(map, "slowhostdays")),
        host_time:
            try!(json_helpers::find_string(map, "hosttime")),
        last_backup_path:
            try!(json_helpers::find_string(map, "lastbackuppath")),
        next_host_datetime:
            try!(json_helpers::find_string(map, "nexthost")),
        all_turns_in:
            try!(json_helpers::find_bool(map, "allturnsin")),
        last_notified:
            try!(json_helpers::find_bool(map, "lastnotified")),
        is_hosting:
            try!(json_helpers::find_bool(map, "ishosting")),
        last_loaded_datetime:
            try!(json_helpers::find_string(map, "lastloadeddate")),
        deleted_datetime:
            try!(json_helpers::find_string(map, "deletedate")),
        last_host_datetime:
            try!(json_helpers::find_string(map, "lasthostdate")),
        password:
            try!(json_helpers::find_string(map, "password")),
        has_password:
            try!(json_helpers::find_bool(map, "haspassword")),
        status_name:
            try!(json_helpers::find_string(map, "statusname")),
        just_ended:
            try!(json_helpers::find_bool(map, "justended")),
        time_to_host:
            try!(json_helpers::find_string(map, "timetohost")),
        id:
            try!(json_helpers::find_i64(map, "id")),
    })
}

#[test]
fn test_build() {
    let json = "{\
        \"name\":\"Vevalgoz Sector\",\
        \"description\":\"This is a battle for the Vevalgoz Sector. This is a default configuration game. Custom map. This game has 2 turns per week.\",\
        \"shortdescription\":\"\",\
        \"status\":2,\
        \"datecreated\":\"10\\/8\\/2011 12:04:45 PM\",\
        \"dateended\":\"1\\/1\\/0001 12:00:00 AM\",\
        \"maptype\":2,\
        \"gametype\":2,\
        \"wincondition\":1,\
        \"difficulty\":0.881292261457551,\
        \"tutorialid\":0,\
        \"requiredlevelid\":0,\
        \"maxlevelid\":0,\
        \"masterplanetid\":257,\
        \"quadrant\":20,\
        \"mintenacity\":0,\
        \"faststart\":0,\
        \"turnsperweek\":2,\
        \"yearstarted\":10,\
        \"isprivate\":false,\
        \"scenarioid\":0,\
        \"createdby\":\"none\",\
        \"turn\":327,\
        \"slots\":11,\
        \"turnstatus\":\"x2_x_x_x9_B\",\
        \"hostdays\":\"S___T__\",\
        \"slowhostdays\":\"\",\
        \"hosttime\":\"2:27\",\
        \"lastbackuppath\":\"c:\\\\planetsdata\\\\backups\\\\game25164\\\\turn326-635492683417604402.zip\",\
        \"nexthost\":\"10\\/23\\/2014 2:27:00 AM\",\
        \"allturnsin\":false,\
        \"lastnotified\":false,\
        \"ishosting\":false,\
        \"lastloadeddate\":\"10\\/21\\/2014 8:12:55 PM\",\
        \"deletedate\":\"\",\
        \"lasthostdate\":\"10\\/18\\/2014 10:36:09 PM\",\
        \"password\":\"\",\
        \"haspassword\":false,\
        \"statusname\":\"Running\",\
        \"justended\":false,\
        \"timetohost\":\"Next turn in 27 hours\",\
        \"id\":25164\
    }";
    let result = Game {
        name: "Vevalgoz Sector".to_string(),
        description:
            "This is a battle for the Vevalgoz Sector. This is a default configuration game. \
            Custom map. This game has 2 turns per week.".to_string(),
        short_description: "".to_string(),
        status: 2i64,
        created_datetime: "10/8/2011 12:04:45 PM".to_string(),
        ended_datetime: "1/1/0001 12:00:00 AM".to_string(),
        map_type: 2i64,
        game_type: 2i64,
        win_condition: 1i64,
        difficulty: "0.881292261457551".to_string(),
        tutorial_id: 0i64,
        required_level_id: 0i64,
        max_level_id: 0i64,
        master_planet_id: 257i64,
        quadrant: 20i64,
        min_tenacity: 0i64,
        fast_start: 0i64,
        turns_per_week: 2i64,
        year_started: 10i64,
        is_private: false,
        scenario_id: 0i64,
        created_by: "none".to_string(),
        turn: 327i64,
        slots: 11i64,
        turn_status: "x2_x_x_x9_B".to_string(),
        host_days: "S___T__".to_string(),
        slow_host_days: "".to_string(),
        host_time: "2:27".to_string(),
        last_backup_path: "c:\\planetsdata\\backups\\game25164\\turn326-635492683417604402.zip".to_string(),
        next_host_datetime: "10/23/2014 2:27:00 AM".to_string(),
        all_turns_in: false,
        last_notified: false,
        is_hosting: false,
        last_loaded_datetime: "10/21/2014 8:12:55 PM".to_string(),
        deleted_datetime: "".to_string(),
        last_host_datetime: "10/18/2014 10:36:09 PM".to_string(),
        password: "".to_string(),
        has_password: false,
        status_name: "Running".to_string(),
        just_ended: false,
        time_to_host: "Next turn in 27 hours".to_string(),
        id: 25164i64,
    };
    assert_eq!(result, build(&common::json_to_map(json).unwrap()).unwrap());
}
