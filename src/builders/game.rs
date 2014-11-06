extern crate serialize;

use self::serialize::json;

use error;
use json_helpers::{find, get_bool, get_float, get_i32, get_object, get_string};

macro_rules! get(
    ($i1:ident, $e:expr, $i2:ident) => (try!($i2(try!(find($i1, $e)))))
)

// Public

#[deriving(Eq, PartialEq, Show)]
pub struct Game {
    pub name: String,
    pub description: String,
    pub short_description: String,
    pub status: i32,
    pub created_datetime: String,
    pub ended_datetime: String,
    pub map_type: i32,
    pub game_type: i32,
    pub win_condition: i32,
    pub difficulty: String,
    pub tutorial_id: i32,
    pub required_level_id: i32,
    pub max_level_id: i32,
    pub master_planet_id: i32,
    pub quadrant: i32,
    pub min_tenacity: i32,
    pub fast_start: i32,
    pub turns_per_week: i32,
    pub year_started: i32,
    pub is_private: bool,
    pub scenario_id: i32,
    pub created_by: String,
    pub turn: i32,
    pub slots: i32,
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
    pub id: i32,
}

pub fn build(json: &json::Json) -> Result<Game, error::Error> {
    let map = try!(get_object(json));
    Ok(Game {
        name:                 get!(map, "name", get_string),
        description:          get!(map, "description", get_string),
        short_description:    get!(map, "shortdescription", get_string),
        status:               get!(map, "status", get_i32),
        created_datetime:     get!(map, "datecreated", get_string),
        ended_datetime:       get!(map, "dateended", get_string),
        map_type:             get!(map, "maptype", get_i32),
        game_type:            get!(map, "gametype", get_i32),
        win_condition:        get!(map, "wincondition", get_i32),
        difficulty:           try!(get_float(try!(find(map, "difficulty")), 15u)),
        tutorial_id:          get!(map, "tutorialid", get_i32),
        required_level_id:    get!(map, "requiredlevelid", get_i32),
        max_level_id:         get!(map, "maxlevelid", get_i32),
        master_planet_id:     get!(map, "masterplanetid", get_i32),
        quadrant:             get!(map, "quadrant", get_i32),
        min_tenacity:         get!(map, "mintenacity", get_i32),
        fast_start:           get!(map, "faststart", get_i32),
        turns_per_week:       get!(map, "turnsperweek", get_i32),
        year_started:         get!(map, "yearstarted", get_i32),
        is_private:           get!(map, "isprivate", get_bool),
        scenario_id:          get!(map, "scenarioid", get_i32),
        created_by:           get!(map, "createdby", get_string),
        turn:                 get!(map, "turn", get_i32),
        slots:                get!(map, "slots", get_i32),
        turn_status:          get!(map, "turnstatus", get_string),
        host_days:            get!(map, "hostdays", get_string),
        slow_host_days:       get!(map, "slowhostdays", get_string),
        host_time:            get!(map, "hosttime", get_string),
        last_backup_path:     get!(map, "lastbackuppath", get_string),
        next_host_datetime:   get!(map, "nexthost", get_string),
        all_turns_in:         get!(map, "allturnsin", get_bool),
        last_notified:        get!(map, "lastnotified", get_bool),
        is_hosting:           get!(map, "ishosting", get_bool),
        last_loaded_datetime: get!(map, "lastloadeddate", get_string),
        deleted_datetime:     get!(map, "deletedate", get_string),
        last_host_datetime:   get!(map, "lasthostdate", get_string),
        password:             get!(map, "password", get_string),
        has_password:         get!(map, "haspassword", get_bool),
        status_name:          get!(map, "statusname", get_string),
        just_ended:           get!(map, "justended", get_bool),
        time_to_host:         get!(map, "timetohost", get_string),
        id:                   get!(map, "id", get_i32),
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
            status: 2,
            created_datetime: "10/8/2011 12:04:45 PM".to_string(),
            ended_datetime: "1/1/0001 12:00:00 AM".to_string(),
            map_type: 2,
            game_type: 2,
            win_condition: 1,
            difficulty: "0.881292261457551".to_string(),
            tutorial_id: 0,
            required_level_id: 0,
            max_level_id: 0,
            master_planet_id: 257,
            quadrant: 20,
            min_tenacity: 0,
            fast_start: 0,
            turns_per_week: 2,
            year_started: 10,
            is_private: false,
            scenario_id: 0,
            created_by: "none".to_string(),
            turn: 327,
            slots: 11,
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
            id: 25164,
        };
        assert_eq!(result, build(&parse(json).unwrap()).unwrap());
    }
}
