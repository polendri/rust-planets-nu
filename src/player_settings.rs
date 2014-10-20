extern crate serialize;

use self::serialize::json;
use std::collections;

use common;
use common::RGB;

#[deriving(Eq, PartialEq, Show)]
pub struct PlayerSettings {
    pub player_planet_colors: (RGB, RGB),
    /*
    pub enemy_planet_colors: (RGB, RGB),
    pub ally_planet_colors: (RGB, RGB),
    pub info_planet_colors: (RGB, RGB),
    pub unknown_planet_colors: (RGB, RGB),
    pub player_ship_colors: (RGB, RGB),
    pub enemy_ship_colors: (RGB, RGB),
    pub ally_ship_colors: (RGB, RGB),
    pub player_mine_color: RGB,
    pub enemy_mine_color: RGB,
    pub web_mine_color: RGB,
    pub ally_mine_color: RGB,
    pub ion_storm_color: RGB,
    pub assistant_enabled: bool,
    pub mouse_zoom_enabled: bool,
    pub sound_effects_enabled: bool,
    pub music_enabled: bool,
    pub battle_task_id: i64,
    pub battle_tutorial_id: i64,
    pub id: i64,
    */
}

pub fn read(map: &collections::TreeMap<String, json::Json>) -> Result<PlayerSettings, common::Error> {
    Ok(PlayerSettings {
        player_planet_colors: (
            try!(common::to_rgb(match_json_string!(map, "myplanetfrom").as_slice())),
            try!(common::to_rgb(match_json_string!(map, "myplanetto").as_slice()))),
    })
}

#[test]
fn test_read() {
    let json = "{\
        \"myplanetfrom\" : \"#ccffff\",\
        \"myplanetto\" : \"#00ffff\"\
    }";
    let result = PlayerSettings {
        player_planet_colors: (
            RGB { red: 204u8, green: 255u8, blue: 255u8 },
            RGB { red: 0u8, green: 255u8, blue: 255u8 }),
    };
    assert_eq!(result, read(&common::json_to_map(json).unwrap()).unwrap());
}
