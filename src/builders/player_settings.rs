/*!
Structs and functions for reading player settings data.
*/
extern crate serialize;

use self::serialize::json;
use std::collections;

use builders::helpers;
use builders::helpers::RGB;
use error;
use json_helpers::find;
use json_helpers::get_bool;
use json_helpers::get_float;
use json_helpers::get_i64;
use json_helpers::get_object;
use json_helpers::get_string;
use json_helpers::parse;

/// Represents the player settings data structure.
#[deriving(Eq, PartialEq, Show)]
pub struct PlayerSettings {
    pub player_planet_colors: (RGB, RGB),
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
}

pub fn build(json: &json::Json) -> Result<PlayerSettings, error::Error> {
    let map = try!(get_object(json));
    Ok(PlayerSettings {
        player_planet_colors: (
            try!(helpers::to_rgb(try!(get_string(try!(find(map, "myplanetfrom")))).as_slice())),
            try!(helpers::to_rgb(try!(get_string(try!(find(map, "myplanetto")))).as_slice()))),
        enemy_planet_colors: (
            try!(helpers::to_rgb(try!(get_string(try!(find(map, "enemyplanetfrom")))).as_slice())),
            try!(helpers::to_rgb(try!(get_string(try!(find(map, "enemyplanetto")))).as_slice()))),
        ally_planet_colors: (
            try!(helpers::to_rgb(try!(get_string(try!(find(map, "allyplanetfrom")))).as_slice())),
            try!(helpers::to_rgb(try!(get_string(try!(find(map, "allyplanetto")))).as_slice()))),
        info_planet_colors: (
            try!(helpers::to_rgb(try!(get_string(try!(find(map, "infoplanetfrom")))).as_slice())),
            try!(helpers::to_rgb(try!(get_string(try!(find(map, "infoplanetto")))).as_slice()))),
        unknown_planet_colors: (
            try!(helpers::to_rgb(try!(get_string(try!(find(map, "unknownplanetfrom")))).as_slice())),
            try!(helpers::to_rgb(try!(get_string(try!(find(map, "unknownplanetto")))).as_slice()))),
        player_ship_colors: (
            try!(helpers::to_rgb(try!(get_string(try!(find(map, "myshipfrom")))).as_slice())),
            try!(helpers::to_rgb(try!(get_string(try!(find(map, "myshipto")))).as_slice()))),
        enemy_ship_colors: (
            try!(helpers::to_rgb(try!(get_string(try!(find(map, "enemyshipfrom")))).as_slice())),
            try!(helpers::to_rgb(try!(get_string(try!(find(map, "enemyshipto")))).as_slice()))),
        ally_ship_colors: (
            try!(helpers::to_rgb(try!(get_string(try!(find(map, "allyshipfrom")))).as_slice())),
            try!(helpers::to_rgb(try!(get_string(try!(find(map, "allyshipto")))).as_slice()))),
        player_mine_color:
            try!(helpers::to_rgb(try!(get_string(try!(find(map, "mymines")))).as_slice())),
        enemy_mine_color:
            try!(helpers::to_rgb(try!(get_string(try!(find(map, "enemymines")))).as_slice())),
        web_mine_color:
            try!(helpers::to_rgb(try!(get_string(try!(find(map, "webmines")))).as_slice())),
        ally_mine_color:
            try!(helpers::to_rgb(try!(get_string(try!(find(map, "allymines")))).as_slice())),
        ion_storm_color:
            try!(helpers::to_rgb(try!(get_string(try!(find(map, "ionstorms")))).as_slice())),
        assistant_enabled:
            try!(get_bool(try!(find(map, "assistanton")))),
        mouse_zoom_enabled:
            try!(get_bool(try!(find(map, "mousezoom")))),
        sound_effects_enabled:
            try!(get_bool(try!(find(map, "soundon")))),
        music_enabled:
            try!(get_bool(try!(find(map, "musicon")))),
        battle_task_id:
            try!(get_i64(try!(find(map, "battletaskid")))),
        battle_tutorial_id:
            try!(get_i64(try!(find(map, "battletutorialid")))),
        id:
            try!(get_i64(try!(find(map, "id")))),
    })
}

#[test]
fn test_build() {
    let json = "{\
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
    }";
    let result = PlayerSettings {
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
    };
    assert_eq!(result, build(&parse(json).unwrap()).unwrap());
}
