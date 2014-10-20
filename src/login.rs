extern crate serialize;

use self::serialize::json;
use std::collections;

use common;
use player_settings;
use player_settings::PlayerSettings;

#[deriving(Eq, PartialEq, Show)]
pub struct LoginResult {
    pub api_key: String,
    pub player_settings: PlayerSettings,
}

pub fn read(map: &collections::TreeMap<String, json::Json>) -> Result<LoginResult, common::Error> {
    Ok(LoginResult {
        api_key: match_json_string!(map, "apikey"),
        player_settings: try!(player_settings::read(match_json_object!(map, "settings"))),
    })
}

#[test]
fn test_read() {
    let json = "{\
        \"apikey\" : \"2ee5dbee-55a8-45e1-ab16-2f72b59c0158\",\
        \"settings\" : {\
            \"myplanetfrom\" : \"#ccffff\",\
            \"myplanetto\" : \"#00ffff\"\
        },\
        \"success\" : true\
    }";
    let result = LoginResult {
        api_key: "2ee5dbee-55a8-45e1-ab16-2f72b59c0158".to_string(),
        player_settings: PlayerSettings {
            player_planet_colors: (
                common::RGB { red: 204u8, green: 255u8, blue: 255u8 },
                common::RGB { red: 0u8, green: 255u8, blue: 255u8 }),
        },
    };
    assert_eq!(result, read(&common::json_to_map(json).unwrap()).unwrap());
}
