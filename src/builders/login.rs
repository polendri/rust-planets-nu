/*!
Structs and functions for reading login data.
*/
extern crate serialize;

use self::serialize::json;
use std::collections;

#[allow(unused_imports)] // common is used for testing
use common;
#[allow(unused_imports)] // RGB is used for testing
use common::RGB;
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
            \"allymines\" : \"#99ff99\",\
            \"allyplanetfrom\" : \"#9999ff\",\
            \"allyplanetto\" : \"#333399\",\
            \"allyshipfrom\" : \"#99ff99\",\
            \"allyshipto\" : \"#669966\",\
            \"assistanton\" : true,\
            \"battletaskid\" : 0,\
            \"battletutorialid\" : 1,\
            \"enemymines\" : \"#ff0000\",\
            \"enemyplanetfrom\" : \"#ffcccc\",\
            \"enemyplanetto\" : \"#ff0000\",\
            \"enemyshipfrom\" : \"#ff0000\",\
            \"enemyshipto\" : \"#990000\",\
            \"id\" : 0,\
            \"infoplanetfrom\" : \"#ffffcc\",\
            \"infoplanetto\" : \"#993300\",\
            \"ionstorms\" : \"#ffff00\",\
            \"mousezoom\" : true,\
            \"musicon\" : \"true\",\
            \"mymines\" : \"#00ff00\",\
            \"myplanetfrom\" : \"#ccffff\",\
            \"myplanetto\" : \"#00ffff\",\
            \"myshipfrom\" : \"#00ff00\",\
            \"myshipto\" : \"#009900\",\
            \"soundon\" : \"true\",\
            \"unknownplanetfrom\" : \"#ffffff\",\
            \"unknownplanetto\" : \"#505050\",\
            \"webmines\" : \"#f000f0\"\
        },\
        \"success\" : true\
    }";
    let result = LoginResult {
        api_key: "2ee5dbee-55a8-45e1-ab16-2f72b59c0158".to_string(),
        player_settings: PlayerSettings {
            player_planet_colors: (
                RGB { red: 0xccu8, green: 0xffu8, blue: 0xffu8 },
                RGB { red: 0x00u8, green: 0xffu8, blue: 0xffu8 }),
            enemy_planet_colors: (
                RGB { red: 0xffu8, green: 0xccu8, blue: 0xccu8 },
                RGB { red: 0xffu8, green: 0x00u8, blue: 0x00u8 }),
            ally_planet_colors: (
                RGB { red: 0x99u8, green: 0x99u8, blue: 0xffu8 },
                RGB { red: 0x33u8, green: 0x33u8, blue: 0x99u8 }),
            info_planet_colors: (
                RGB { red: 0xffu8, green: 0xffu8, blue: 0xccu8 },
                RGB { red: 0x99u8, green: 0x33u8, blue: 0x00u8 }),
            unknown_planet_colors: (
                RGB { red: 0xffu8, green: 0xffu8, blue: 0xffu8 },
                RGB { red: 0x50u8, green: 0x50u8, blue: 0x50u8 }),
            player_ship_colors: (
                RGB { red: 0x00u8, green: 0xffu8, blue: 0x00u8 },
                RGB { red: 0x00u8, green: 0x99u8, blue: 0x00u8 }),
            enemy_ship_colors: (
                RGB { red: 0xffu8, green: 0x00u8, blue: 0x00u8 },
                RGB { red: 0x99u8, green: 0x00u8, blue: 0x00u8 }),
            ally_ship_colors: (
                RGB { red: 0x99u8, green: 0xffu8, blue: 0x99u8 },
                RGB { red: 0x66u8, green: 0x99u8, blue: 0x66u8 }),
            player_mine_color: 
                RGB { red: 0x00u8, green: 0xffu8, blue: 0x00u8 },
            enemy_mine_color: 
                RGB { red: 0xffu8, green: 0x00u8, blue: 0x00u8 },
            web_mine_color: 
                RGB { red: 0xf0u8, green: 0x00u8, blue: 0xf0u8 },
            ally_mine_color: 
                RGB { red: 0x99u8, green: 0xffu8, blue: 0x99u8 },
            ion_storm_color: 
                RGB { red: 0xffu8, green: 0xffu8, blue: 0x00u8 },
            assistant_enabled: true,
            mouse_zoom_enabled: true,
            sound_effects_enabled: true,
            music_enabled: true,
            battle_task_id: 0,
            battle_tutorial_id: 1,
            id: 0,
        },
    };
    assert_eq!(result, build(&common::json_to_map(json).unwrap()).unwrap());
}
