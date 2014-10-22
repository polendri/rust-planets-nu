extern crate serialize;

use self::serialize::json;

use builders::game;
use builders::game::Game;
use common;
use error;
use json_helpers;

/// Builds a game struct given a JSON object map.
pub fn build(vec: &Vec<json::Json>) -> Result<Vec<Game>, error::Error> {
    let mut results: Vec<Game> = Vec::with_capacity(vec.len());

    for game in vec.iter() {
        let map = match *game {
            json::Object(ref x) => x,
            _ => return Err(error::Error::new(
                error::LibError,
                "Expected object type in list but found something else.".to_string())),
        };
        results.push(try!(game::build(map)));
    }
    Ok(results)
}

#[test]
fn test_build() {
    let json = "[{\
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
    }]";
    let expected = Game {
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

    let results: Vec<Game> = build(&common::json_to_list(json).unwrap()).unwrap();
    assert_eq!(1, results.len());
    //assert_eq!(expected, results[0]);
}
