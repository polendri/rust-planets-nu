extern crate encoding;
extern crate serialize;

use self::serialize::json;
use std::char;
use std::cmp;
use std::str;

use curl;

// Macros

macro_rules! find_match(
    ($i:ident, $e1:expr, $p:pat => $e2:expr) => (try_match!(*expect!($i.find(&$e1.to_string()), str::Owned(format!("Could not find key '{}'.", $e1))), $p => $e2, str::Owned(format!("Unexpected type for key '{}'.", $e1))))
)

// Common structs

pub struct RGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

// Login structs

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

/*
pub struct LoginRequest<'a> {
    pub username: &'a str,
    pub password: &'a str,
}
*/

#[deriving(Eq, PartialEq, Show)]
pub struct LoginResult {
    pub api_key: String,
    //pub player_settings: PlayerSettings,
}

/*
// Game Info structs

pub struct Game<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub short_description: &'a str,
    pub status: i64,
    pub created_datetime: &'a str, // datetime
    pub ended_datetime: &'a str, // datetime
    pub map_type: i64,
    pub game_type: i64,
    pub win_condition: i64,
    pub difficulty: f64,
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
    pub created_by: &'a str,
    pub turn: i64,
    pub slots: i64,
    pub turn_status: &'a str, // its own datatype?
    pub host_days: &'a str, // its own datatype?
    pub slow_host_days: &'a str, // its own datatype?
    pub host_time: &'a str, // time
    pub last_backup_path: &'a str,
    pub next_host_datetime: &'a str, // datetime
    pub all_turns_in: bool,
    pub last_notified: bool,
    pub is_hosting: bool,
    pub last_loaded_datetime: &'a str, // datetime
    pub deleted_datetime: &'a str, // datetime
    pub last_host_datetime: &'a str, // datetime
    pub password: &'a str,
    pub has_password: bool,
    pub status_name: &'a str,
    pub just_ended: bool,
    pub time_to_host: &'a str,
    pub id: i64,
}

pub struct GameSettings<'a> {
    pub name: &'a str,
    pub turn: i64,
    pub build_queue_planet_id: i64,
    pub victory_countdown: i64,
    pub max_allies: i64,
    pub map_width: i64,
    pub map_height: i64,
    pub num_planets: i64,
    pub ship_limit: i64,
    pub host_start_datetime: &'a str,
    pub host_completed_datetime: &'a str,
    pub next_host_datetime: &'a str,
    pub last_invite_datetime: &'a str,
    pub team_size: i64,
    pub planet_scan_range: i64,
    pub ship_scan_range: i64,
    pub all_visible: bool,
    pub minefields_visible: bool,
    pub nebulas: i64,
    pub stars: i64,
    pub discussion_id: &'a str,
    pub nu_ion_storms: bool,
    pub max_ion_storms: i64,
    pub max_ion_clouds_per_storm: i64,
    pub debris_disk_percent: i64,
    pub debris_disk_version: i64,
    pub cloak_fail: i64,
    pub structure_decay_rate: i64,
    pub map_shape: i64,
    pub very_close_planets: i64,
    pub close_planets: i64,
    pub other_planets_min_homeworld_dist: i64,
    pub n_circles: i64, // ?
    pub hw_distribution: i64, // ?
    pub n_debris_discs: i64, // ?
    pub level_id: i64,
    pub next_level_id: i64,
    pub kill_race: bool,
    pub running_start: i64,
    pub dead_radius: i64,
    pub player_select_race: bool,
    pub military_score_percent: i64,
    pub hide_race_selection: bool,
    pub fixed_start_positions: bool,
    pub min_native_clans: i64,
    pub max_native_clans: i64,
    pub homeworld_has_starbase: bool,
    pub homeworld_clans: i64,
    pub homeworld_resources: i64,
    pub game_password: &'a str,
    pub neutronium_level: f64,
    pub duranium_level: f64,
    pub tritanium_level: f64,
    pub molybdenum_level: f64,
    pub average_density_percent: i64,
    pub development_factor: i64,
    pub native_probability: i64,
    pub native_government_level: i64,
    pub max_surface_neutronium: i64,
    pub max_surface_duranium: i64,
    pub max_surface_tritanium: i64,
    pub max_surface_molybdenum: i64,
    pub max_ground_neutronium: i64,
    pub max_ground_duranium: i64,
    pub max_ground_tritanium: i64,
    pub max_ground_molybdenum: i64,
    pub computer_build_ships: bool,
    pub computer_build_delay: i64,
    pub fight_or_fail: i64,
    pub show_all_explosions: bool,
    pub campaign_mode: bool,
    pub max_advantage: i64,
    pub fascist_double_beams: bool,
    pub production_queue: bool,
    pub production_base_cost: i64,
    pub production_starbase_output: i64,
    pub production_starbase_reward: i64,
    pub end_turn: i64,
    pub id: i64,
}

pub struct PlayerScore<'a> {
    pub added_datetime: &'a str,
    pub owner_id: i64,
    pub account_id: i64,
    pub capital_ships: i64,
    pub freighters: i64,
    pub planets: i64,
    pub starbases: i64,
    pub military_score: i64,
    pub inventory_score: i64,
    pub priority_points: i64,
    pub turn: i64,
    pub percent: f64,
    pub id: i64,
}

pub struct PlayerInfo<'a> {
    pub status: i64,
    pub status_turn: i64,
    pub account_id: i64,
    pub username: &'a str,
    pub email: &'a str,
    pub race_id: i64,
    pub team_id: i64,
    pub priority_points: i64,
    pub join_rank: i64,
    pub finish_rank: i64,
    pub turn_joined: i64,
    pub turn_ready: i64,
    pub turn_ready_datetime: &'a str,
    pub turn_status: i64,
    pub turns_missed: i64,
    pub turns_missed_total: i64,
    pub turns_holiday: i64,
    pub turns_early: i64,
    pub active_hulls: &'a [i64],
    pub active_advantages: &'a [i64],
    pub save_key: &'a str,
    pub tutorial_id: i64,
    pub tutorial_task_id: i64,
    pub is_registered: bool,
    pub level_id: i64,
    pub level_hull_id: i64,
    pub player_score: PlayerScore<'a>,
    pub planets: i64,
    pub ships: i64,
    pub starbases: i64,
    pub id: i64,
}

pub struct Relation {
    pub player_id: i64,
    pub player_to_id: i64,
    pub relation_to: i64,
    pub relation_from: i64,
    pub conflict_level: i64,
    pub color: RGB,
    pub id: i64,
}

pub struct GameInfo<'a> {
    pub game: Game<'a>,
    pub year_from: i64,
    pub year_to: Option<i64>,
    pub win_condition: &'a str,
    pub schedule: &'a str,
    pub time_to_host: &'a str,
    pub has_password: bool,
    pub settings: GameSettings<'a>,
    pub player_infos: &'a [PlayerInfo<'a>],
    pub relations: &'a [Relation],
}
*/

fn to_rgb(rgb_str: &str) -> Result<RGB, str::MaybeOwned> {
    let len_msg = str::Slice("RGB string is too short.");
    let digit_msg = str::Slice("Encountered a non-hex digit.");
    if rgb_str.len() != 7 {
        return Err(str::Slice("Unexpected RGB string length."));
    }
    let mut iter = rgb_str.chars();
    try_match!(expect!(iter.next(), len_msg), '#' => (), str::Slice("Could not find leading '#'."));
    let r = (expect!(char::to_digit(expect!(iter.next(), len_msg), 16), digit_msg) << 4) +
        expect!(char::to_digit(expect!(iter.next(), len_msg), 16), digit_msg);
    let g = (expect!(char::to_digit(expect!(iter.next(), len_msg), 16), digit_msg) << 4) +
        expect!(char::to_digit(expect!(iter.next(), len_msg), 16), digit_msg);
    let b = (expect!(char::to_digit(expect!(iter.next(), len_msg), 16), digit_msg) << 4) +
        expect!(char::to_digit(expect!(iter.next(), len_msg), 16), digit_msg);
    Ok(RGB { red: r as u8, green: g as u8, blue: b as u8 })
}

fn decode_login_response(json: &str) -> Result<LoginResult, str::MaybeOwned> {
    let root_enum = match json::from_str(json) {
        Ok(x) => x,
        Err(error) => return Err(str::Owned(format!("Error while decoding the login response: {}", error))),
    };
    let root_tree = try_match!(root_enum, json::Object(x) => x, str::Slice("Could not find root of login response"));
    let success = find_match!(root_tree, "success", json::Boolean(x) => x);
    if !success {
        return Err(str::Slice("Response indicates failure."));
    };
    Ok(LoginResult {
        api_key: find_match!(root_tree, "apikey", json::String(ref x) => x.clone()),
    })
}

/*
pub fn game_info_json(game_id: i64) -> Result<String, String> {
    let url = "http://api.planets.nu/game/loadinfo?gameid=".to_string() + game_id.to_string();
*/

#[test]
fn test_decode_login_response() {
    let json = "{ \"apikey\" : \"2ee5dbee-55a8-45e1-ab16-2f72b59c0158\", \"success\" : true }";
    let result = LoginResult { api_key: "2ee5dbee-55a8-45e1-ab16-2f72b59c0158".to_string() };
    assert_eq!(result, decode_login_response(json).unwrap());
}
