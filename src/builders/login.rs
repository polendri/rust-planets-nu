/*!
Structs and functions for reading login data.
*/
extern crate serialize;

use self::serialize::json;
use std::collections;

#[allow(unused_imports)] // common is used for testing
use common;
use error;
use builders::player_settings;
use builders::player_settings::PlayerSettings;

/// Represents the login result data structure.
#[deriving(Eq, PartialEq, Show)]
pub struct LoginResult {
    pub api_key: String,
    pub player_settings: PlayerSettings,
}

/// Builds a login result struct given a JSON object map.
pub fn build(map: &collections::TreeMap<String, json::Json>) -> Result<LoginResult, error::Error> {
    Ok(LoginResult {
        api_key: match_json_string!(map, "apikey"),
        player_settings: try!(player_settings::build(match_json_object!(map, "settings"))),
    })
}

#[test]
fn test_build() {
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
    assert_eq!(result, build(&common::json_to_map(json).unwrap()).unwrap());
}
