extern crate serialize;

use self::serialize::json;

use builders::helpers;
use builders::player_settings::{mod, PlayerSettings};
use error;
use json_helpers::{find, get_object, get_string};

// Public

/// Contains the data returned in a login API response.
#[deriving(Eq, PartialEq, Show)]
pub struct LoginResult {
    /// The API key for the user, to authenticate in future API calls.
    pub api_key: String,
    /// The settings for the user.
    pub player_settings: PlayerSettings,
}

pub fn build(json: &json::Json) -> Result<LoginResult, error::Error> {
    let map = try!(get_object(json));
    try!(helpers::check_success(map));
    Ok(LoginResult {
        api_key: try!(get_string(try!(find(map, "apikey")))),
        player_settings: try!(player_settings::build(try!(find(map, "settings")))),
    })
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use data::RGB;
    use builders::player_settings::PlayerSettings;
    use json_helpers::parse;

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
        assert_eq!(result, build(&parse(json).unwrap()).unwrap());
    }
}
