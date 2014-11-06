extern crate serialize;

use self::serialize::json;

use error;
use json_helpers::{find, get_bool, get_float, get_i32, get_object, get_string};

// Public

#[deriving(Eq, PartialEq, Show)]
pub struct GameSettings {
    pub name: String,
    pub turn: i32,
    pub build_queue_planet_id: i32,
    pub victory_countdown: i32,
    pub max_allies: i32,
    pub map_width: i32,
    pub map_height: i32,
    pub num_planets: i32,
    pub ship_limit: i32,
    pub host_start_datetime: String,
    pub host_completed_datetime: String,
    pub next_host_datetime: String,
    pub last_invite_datetime: String,
    pub team_size: i32,
    pub planet_scan_range: i32,
    pub ship_scan_range: i32,
    pub all_visible: bool,
    pub minefields_visible: bool,
    pub nebulas: i32,
    pub stars: i32,
    pub discussion_id: String,
    pub nu_ion_storms: bool,
    pub max_ion_storms: i32,
    pub max_ion_clouds_per_storm: i32,
    pub debris_disk_percent: i32,
    pub debris_disk_version: i32,
    pub cloak_fail: i32,
    pub structure_decay_rate: i32,
    pub map_shape: i32,
    pub very_close_planets: i32,
    pub close_planets: i32,
    pub other_planets_min_homeworld_dist: i32,
    pub n_circles: i32,
    pub hw_distribution: i32,
    pub n_debris_discs: i32,
    pub level_id: i32,
    pub next_level_id: i32,
    pub kill_race: bool,
    pub running_start: i32,
    pub dead_radius: i32,
    pub player_select_race: bool,
    pub military_score_percent: i32,
    pub hide_race_selection: bool,
    pub fixed_start_positions: bool,
    pub min_native_clans: i32,
    pub max_native_clans: i32,
    pub homeworld_has_starbase: bool,
    pub homeworld_clans: i32,
    pub homeworld_resources: i32,
    pub game_password: String,
    pub neutronium_level: String,
    pub duranium_level: String,
    pub tritanium_level: String,
    pub molybdenum_level: String,
    pub average_density_percent: i32,
    pub development_factor: i32,
    pub native_probability: i32,
    pub native_government_level: i32,
    pub max_surface_neutronium: i32,
    pub max_surface_duranium: i32,
    pub max_surface_tritanium: i32,
    pub max_surface_molybdenum: i32,
    pub max_ground_neutronium: i32,
    pub max_ground_duranium: i32,
    pub max_ground_tritanium: i32,
    pub max_ground_molybdenum: i32,
    pub computer_build_ships: bool,
    pub computer_build_delay: i32,
    pub fight_or_fail: i32,
    pub show_all_explosions: bool,
    pub campaign_mode: bool,
    pub max_advantage: i32,
    pub fascist_double_beams: bool,
    pub production_queue: bool,
    pub production_base_cost: i32,
    pub production_starbase_output: i32,
    pub production_starbase_reward: i32,
    pub end_turn: i32,
    pub id: i32,
}

pub fn build(json: &json::Json) -> Result<GameSettings, error::Error> {
    let map = try!(get_object(json));
    Ok(GameSettings {
        name:
            try!(get_string(try!(find(map, "name")))),
        turn:
            try!(get_i32(try!(find(map, "turn")))),
        build_queue_planet_id:
            try!(get_i32(try!(find(map, "buildqueueplanetid")))),
        victory_countdown:
            try!(get_i32(try!(find(map, "victorycountdown")))),
        max_allies:
            try!(get_i32(try!(find(map, "maxallies")))),
        map_width:
            try!(get_i32(try!(find(map, "mapwidth")))),
        map_height:
            try!(get_i32(try!(find(map, "mapheight")))),
        num_planets:
            try!(get_i32(try!(find(map, "numplanets")))),
        ship_limit:
            try!(get_i32(try!(find(map, "shiplimit")))),
        host_start_datetime:
            try!(get_string(try!(find(map, "hoststart")))),
        host_completed_datetime:
            try!(get_string(try!(find(map, "hostcompleted")))),
        next_host_datetime:
            try!(get_string(try!(find(map, "nexthost")))),
        last_invite_datetime:
            try!(get_string(try!(find(map, "lastinvite")))),
        team_size:
            try!(get_i32(try!(find(map, "teamsize")))),
        planet_scan_range:
            try!(get_i32(try!(find(map, "planetscanrange")))),
        ship_scan_range:
            try!(get_i32(try!(find(map, "shipscanrange")))),
        all_visible:
            try!(get_bool(try!(find(map, "allvisible")))),
        minefields_visible:
            try!(get_bool(try!(find(map, "minefieldsvisible")))),
        nebulas:
            try!(get_i32(try!(find(map, "nebulas")))),
        stars:
            try!(get_i32(try!(find(map, "stars")))),
        discussion_id:
            try!(get_string(try!(find(map, "discussionid")))),
        nu_ion_storms:
            try!(get_bool(try!(find(map, "nuionstorms")))),
        max_ion_storms:
            try!(get_i32(try!(find(map, "maxions")))),
        max_ion_clouds_per_storm:
            try!(get_i32(try!(find(map, "maxioncloudsperstorm")))),
        debris_disk_percent:
            try!(get_i32(try!(find(map, "debrisdiskpercent")))),
        debris_disk_version:
            try!(get_i32(try!(find(map, "debrisdiskversion")))),
        cloak_fail:
            try!(get_i32(try!(find(map, "cloakfail")))),
        structure_decay_rate:
            try!(get_i32(try!(find(map, "structuredecayrate")))),
        map_shape:
            try!(get_i32(try!(find(map, "mapshape")))),
        very_close_planets:
            try!(get_i32(try!(find(map, "verycloseplanets")))),
        close_planets:
            try!(get_i32(try!(find(map, "closeplanets")))),
        other_planets_min_homeworld_dist:
            try!(get_i32(try!(find(map, "otherplanetsminhomeworlddist")))),
        n_circles:
            try!(get_i32(try!(find(map, "ncircles")))),
        hw_distribution:
            try!(get_i32(try!(find(map, "hwdistribution")))),
        n_debris_discs:
            try!(get_i32(try!(find(map, "ndebrisdiscs")))),
        level_id:
            try!(get_i32(try!(find(map, "levelid")))),
        next_level_id:
            try!(get_i32(try!(find(map, "nextlevelid")))),
        kill_race:
            try!(get_bool(try!(find(map, "killrace")))),
        running_start:
            try!(get_i32(try!(find(map, "runningstart")))),
        dead_radius:
            try!(get_i32(try!(find(map, "deadradius")))),
        player_select_race:
            try!(get_bool(try!(find(map, "playerselectrace")))),
        military_score_percent:
            try!(get_i32(try!(find(map, "militaryscorepercent")))),
        hide_race_selection:
            try!(get_bool(try!(find(map, "hideraceselection")))),
        fixed_start_positions:
            try!(get_bool(try!(find(map, "fixedstartpositions")))),
        min_native_clans:
            try!(get_i32(try!(find(map, "minnativeclans")))),
        max_native_clans:
            try!(get_i32(try!(find(map, "maxnativeclans")))),
        homeworld_has_starbase:
            try!(get_bool(try!(find(map, "homeworldhasstarbase")))),
        homeworld_clans:
            try!(get_i32(try!(find(map, "homeworldclans")))),
        homeworld_resources:
            try!(get_i32(try!(find(map, "homeworldresources")))),
        game_password:
            try!(get_string(try!(find(map, "gamepassword")))),
        neutronium_level:
            try!(get_float(try!(find(map, "neutroniumlevel")), 2u)),
        duranium_level:
            try!(get_float(try!(find(map, "duraniumlevel")), 2u)),
        tritanium_level:
            try!(get_float(try!(find(map, "tritaniumlevel")), 2u)),
        molybdenum_level:
            try!(get_float(try!(find(map, "molybdenumlevel")), 2u)),
        average_density_percent:
            try!(get_i32(try!(find(map, "averagedensitypercent")))),
        development_factor:
            try!(get_i32(try!(find(map, "developmentfactor")))),
        native_probability:
            try!(get_i32(try!(find(map, "nativeprobability")))),
        native_government_level:
            try!(get_i32(try!(find(map, "nativegovernmentlevel")))),
        max_surface_neutronium:
            try!(get_i32(try!(find(map, "neusurfacemax")))),
        max_surface_duranium:
            try!(get_i32(try!(find(map, "dursurfacemax")))),
        max_surface_tritanium:
            try!(get_i32(try!(find(map, "trisurfacemax")))),
        max_surface_molybdenum:
            try!(get_i32(try!(find(map, "molsurfacemax")))),
        max_ground_neutronium:
            try!(get_i32(try!(find(map, "neugroundmax")))),
        max_ground_duranium:
            try!(get_i32(try!(find(map, "durgroundmax")))),
        max_ground_tritanium:
            try!(get_i32(try!(find(map, "trigroundmax")))),
        max_ground_molybdenum:
            try!(get_i32(try!(find(map, "molgroundmax")))),
        computer_build_ships:
            try!(get_bool(try!(find(map, "computerbuildships")))),
        computer_build_delay:
            try!(get_i32(try!(find(map, "computerbuilddelay")))),
        fight_or_fail:
            try!(get_i32(try!(find(map, "fightorfail")))),
        show_all_explosions:
            try!(get_bool(try!(find(map, "showallexplosions")))),
        campaign_mode:
            try!(get_bool(try!(find(map, "campaignmode")))),
        max_advantage:
            try!(get_i32(try!(find(map, "maxadvantage")))),
        fascist_double_beams:
            try!(get_bool(try!(find(map, "fascistdoublebeams")))),
        production_queue:
            try!(get_bool(try!(find(map, "productionqueue")))),
        production_base_cost:
            try!(get_i32(try!(find(map, "productionbasecost")))),
        production_starbase_output:
            try!(get_i32(try!(find(map, "productionstarbaseoutput")))),
        production_starbase_reward:
            try!(get_i32(try!(find(map, "productionstarbasereward")))),
        end_turn:
            try!(get_i32(try!(find(map, "endturn")))),
        id:
            try!(get_i32(try!(find(map, "id")))),
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
            \"name\": \"MFers Most Final Universe\",\
            \"turn\": 7,\
            \"buildqueueplanetid\": 0,\
            \"victorycountdown\": 0,\
            \"maxallies\": 2,\
            \"mapwidth\": 2000,\
            \"mapheight\": 2000,\
            \"numplanets\": 500,\
            \"shiplimit\": 500,\
            \"hoststart\": \"10/8/2014 5:10:51 PM\",\
            \"hostcompleted\": \"10/8/2014 5:10:59 PM\",\
            \"nexthost\": \"1/1/0001 12:00:00 AM\",\
            \"lastinvite\": \"1/1/0001 12:00:00 AM\",\
            \"teamsize\": 0,\
            \"planetscanrange\": 10000,\
            \"shipscanrange\": 350,\
            \"allvisible\": false,\
            \"minefieldsvisible\": false,\
            \"nebulas\": 0,\
            \"stars\": 0,\
            \"discussionid\": \"\",\
            \"nuionstorms\": false,\
            \"maxions\": 4,\
            \"maxioncloudsperstorm\": 10,\
            \"debrisdiskpercent\": 50,\
            \"debrisdiskversion\": 2,\
            \"cloakfail\": 0,\
            \"structuredecayrate\": 3,\
            \"mapshape\": 0,\
            \"verycloseplanets\": 4,\
            \"closeplanets\": 15,\
            \"otherplanetsminhomeworlddist\": 155,\
            \"ncircles\": 1,\
            \"hwdistribution\": 2,\
            \"ndebrisdiscs\": 0,\
            \"levelid\": 0,\
            \"nextlevelid\": 0,\
            \"killrace\": false,\
            \"runningstart\": 0,\
            \"deadradius\": 81,\
            \"playerselectrace\": false,\
            \"militaryscorepercent\": 65,\
            \"hideraceselection\": false,\
            \"fixedstartpositions\": false,\
            \"minnativeclans\": 1000,\
            \"maxnativeclans\": 75000,\
            \"homeworldhasstarbase\": true,\
            \"homeworldclans\": 25000,\
            \"homeworldresources\": 3,\
            \"gamepassword\": \"\",\
            \"neutroniumlevel\": 2.08,\
            \"duraniumlevel\": 1.25,\
            \"tritaniumlevel\": 1.8,\
            \"molybdenumlevel\": 1.16,\
            \"averagedensitypercent\": 55,\
            \"developmentfactor\": 1,\
            \"nativeprobability\": 55,\
            \"nativegovernmentlevel\": 2,\
            \"neusurfacemax\": 250,\
            \"dursurfacemax\": 40,\
            \"trisurfacemax\": 50,\
            \"molsurfacemax\": 25,\
            \"neugroundmax\": 700,\
            \"durgroundmax\": 500,\
            \"trigroundmax\": 500,\
            \"molgroundmax\": 200,\
            \"computerbuildships\": true,\
            \"computerbuilddelay\": 0,\
            \"fightorfail\": 0,\
            \"showallexplosions\": false,\
            \"campaignmode\": false,\
            \"maxadvantage\": 500,\
            \"fascistdoublebeams\": false,\
            \"productionqueue\": false,\
            \"productionbasecost\": 1,\
            \"productionstarbaseoutput\": 2,\
            \"productionstarbasereward\": 2,\
            \"endturn\": 100,\
            \"id\": 0\
        }";
        let result = GameSettings {
            name: "MFers Most Final Universe".to_string(),
            turn: 7i32,
            build_queue_planet_id: 0i32,
            victory_countdown: 0i32,
            max_allies: 2i32,
            map_width: 2000i32,
            map_height: 2000i32,
            num_planets: 500i32,
            ship_limit: 500i32,
            host_start_datetime: "10/8/2014 5:10:51 PM".to_string(),
            host_completed_datetime: "10/8/2014 5:10:59 PM".to_string(),
            next_host_datetime: "1/1/0001 12:00:00 AM".to_string(),
            last_invite_datetime: "1/1/0001 12:00:00 AM".to_string(),
            team_size: 0i32,
            planet_scan_range: 10000i32,
            ship_scan_range: 350i32,
            all_visible: false,
            minefields_visible: false,
            nebulas: 0i32,
            stars: 0i32,
            discussion_id: "".to_string(),
            nu_ion_storms: false,
            max_ion_storms: 4i32,
            max_ion_clouds_per_storm: 10i32,
            debris_disk_percent: 50i32,
            debris_disk_version: 2i32,
            cloak_fail: 0i32,
            structure_decay_rate: 3i32,
            map_shape: 0i32,
            very_close_planets: 4i32,
            close_planets: 15i32,
            other_planets_min_homeworld_dist: 155i32,
            n_circles: 1i32,
            hw_distribution: 2i32,
            n_debris_discs: 0i32,
            level_id: 0i32,
            next_level_id: 0i32,
            kill_race: false,
            running_start: 0i32,
            dead_radius: 81i32,
            player_select_race: false,
            military_score_percent: 65i32,
            hide_race_selection: false,
            fixed_start_positions: false,
            min_native_clans: 1000i32,
            max_native_clans: 75000i32,
            homeworld_has_starbase: true,
            homeworld_clans: 25000i32,
            homeworld_resources: 3i32,
            game_password: "".to_string(),
            neutronium_level: "2.08".to_string(),
            duranium_level: "1.25".to_string(),
            tritanium_level: "1.8".to_string(),
            molybdenum_level: "1.16".to_string(),
            average_density_percent: 55i32,
            development_factor: 1i32,
            native_probability: 55i32,
            native_government_level: 2i32,
            max_surface_neutronium: 250i32,
            max_surface_duranium: 40i32,
            max_surface_tritanium: 50i32,
            max_surface_molybdenum: 25i32,
            max_ground_neutronium: 700i32,
            max_ground_duranium: 500i32,
            max_ground_tritanium: 500i32,
            max_ground_molybdenum: 200i32,
            computer_build_ships: true,
            computer_build_delay: 0i32,
            fight_or_fail: 0i32,
            show_all_explosions: false,
            campaign_mode: false,
            max_advantage: 500i32,
            fascist_double_beams: false,
            production_queue: false,
            production_base_cost: 1i32,
            production_starbase_output: 2i32,
            production_starbase_reward: 2i32,
            end_turn: 100i32,
            id: 0i32,
        };
        assert_eq!(result, build(&parse(json).unwrap()).unwrap());
    }
}
