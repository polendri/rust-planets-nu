extern crate serialize;

use self::serialize::json;

use builders::helpers;
use error;
use json_helpers::{find, get_bool, get_i32, get_list, get_object, get_string};

macro_rules! get(
    ($i1:ident, $e:expr, $i2:ident) => (try!($i2(try!(find($i1, $e)))))
)

// Public

#[deriving(Eq, PartialEq, Show)]
pub struct Ship {
    pub friendly_code: String,
    pub name: String,
    pub warp: i32,
    pub position: (i32, i32),
    pub beams: i32,
    pub bays: i32,
    pub torps: i32,
    pub mission: i32,
    pub mission_1_target: i32,
    pub mission_2_target: i32,
    pub enemy: i32,
    pub damage: i32,
    pub crew: i32,
    pub clans: i32,
    pub neutronium: i32,
    pub tritanium: i32,
    pub duranium: i32,
    pub molybdenum: i32,
    pub supplies: i32,
    pub ammo: i32,
    pub megacredits: i32,
    pub transfer_clans: i32,
    pub transfer_neutronium: i32,
    pub transfer_duranium: i32,
    pub transfer_tritanium: i32,
    pub transfer_molybdenum: i32,
    pub transfer_supplies: i32,
    pub transfer_ammo: i32,
    pub transfer_megacredits: i32,
    pub transfer_target_id: i32,
    pub transfer_target_type: i32,
    pub target_position: (i32, i32),
    pub mass: i32,
    pub heading: i32,
    pub turn: i32,
    pub turn_killed: i32,
    pub beam_id: i32,
    pub engine_id: i32,
    pub hull_id: i32,
    pub owner_id: i32,
    pub torpedo_id: i32,
    pub experience: i32,
    pub info_turn: i32,
    pub goal: i32,
    pub goal_target: i32,
    pub goal_target_2: i32,
    pub waypoints: Vec<(i32, i32)>,
    pub history: Vec<(i32, i32)>,
    pub is_cloaked: bool,
    pub ready_status: i32,
    pub id: i32,
}

pub fn build(json: &json::Json) -> Result<Ship, error::Error> {
    let map = try!(get_object(json));
    Ok(Ship {
        friendly_code: get!(map, "friendlycode", get_string),
        name: get!(map, "name", get_string),
        warp: get!(map, "warp", get_i32),
        position: (get!(map, "x", get_i32), get!(map, "y", get_i32)),
        beams: get!(map, "beams", get_i32),
        bays: get!(map, "bays", get_i32),
        torps: get!(map, "torps", get_i32),
        mission: get!(map, "mission", get_i32),
        mission_1_target: get!(map, "mission1target", get_i32),
        mission_2_target: get!(map, "mission2target", get_i32),
        enemy: get!(map, "enemy", get_i32),
        damage: get!(map, "damage", get_i32),
        crew: get!(map, "crew", get_i32),
        clans: get!(map, "clans", get_i32),
        neutronium: get!(map, "neutronium", get_i32),
        tritanium: get!(map, "tritanium", get_i32),
        duranium: get!(map, "duranium", get_i32),
        molybdenum: get!(map, "molybdenum", get_i32),
        supplies: get!(map, "supplies", get_i32),
        ammo: get!(map, "ammo", get_i32),
        megacredits: get!(map, "megacredits", get_i32),
        transfer_clans: get!(map, "transferclans", get_i32),
        transfer_neutronium: get!(map, "transferneutronium", get_i32),
        transfer_duranium: get!(map, "transferduranium", get_i32),
        transfer_tritanium: get!(map, "transfertritanium", get_i32),
        transfer_molybdenum: get!(map, "transfermolybdenum", get_i32),
        transfer_supplies: get!(map, "transfersupplies", get_i32),
        transfer_ammo: get!(map, "transferammo", get_i32),
        transfer_megacredits: get!(map, "transfermegacredits", get_i32),
        transfer_target_id: get!(map, "transfertargetid", get_i32),
        transfer_target_type: get!(map, "transfertargettype", get_i32),
        target_position: (get!(map, "targetx", get_i32), get!(map, "targety", get_i32)),
        mass: get!(map, "mass", get_i32),
        heading: get!(map, "heading", get_i32),
        turn: get!(map, "turn", get_i32),
        turn_killed: get!(map, "turnkilled", get_i32),
        beam_id: get!(map, "beamid", get_i32),
        engine_id: get!(map, "engineid", get_i32),
        hull_id: get!(map, "hullid", get_i32),
        owner_id: get!(map, "ownerid", get_i32),
        torpedo_id: get!(map, "torpedoid", get_i32),
        experience: get!(map, "experience", get_i32),
        info_turn: get!(map, "infoturn", get_i32),
        goal: get!(map, "goal", get_i32),
        goal_target: get!(map, "goaltarget", get_i32),
        goal_target_2: get!(map, "goaltarget2", get_i32),
        waypoints: try!(helpers::map_with_err(
            get!(map, "waypoints", get_list),
            |x| helpers::get_coordinates(x))),
        history: try!(helpers::map_with_err(
            get!(map, "history", get_list),
            |x| helpers::get_coordinates(x))),
        is_cloaked: get!(map, "iscloaked", get_bool),
        ready_status: get!(map, "readystatus", get_i32),
        id: get!(map, "id", get_i32),
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
            \"friendlycode\": \"\",\
            \"name\": \"Medium Deep Space Freighter\",\
            \"warp\": 9,\
            \"x\": 1885,\
            \"y\": 1297,\
            \"beams\": 0,\
            \"bays\": 0,\
            \"torps\": 0,\
            \"mission\": 0,\
            \"mission1target\": 0,\
            \"mission2target\": 0,\
            \"enemy\": 0,\
            \"damage\": -1,\
            \"crew\": -1,\
            \"clans\": 0,\
            \"neutronium\": 0,\
            \"tritanium\": 0,\
            \"duranium\": 0,\
            \"molybdenum\": 0,\
            \"supplies\": 0,\
            \"ammo\": 0,\
            \"megacredits\": 0,\
            \"transferclans\": 0,\
            \"transferneutronium\": 0,\
            \"transferduranium\": 0,\
            \"transfertritanium\": 0,\
            \"transfermolybdenum\": 0,\
            \"transfersupplies\": 0,\
            \"transferammo\": 0,\
            \"transfermegacredits\": 0,\
            \"transfertargetid\": 0,\
            \"transfertargettype\": 0,\
            \"targetx\": 1885,\
            \"targety\": 1300,\
            \"mass\": 212,\
            \"heading\": 0,\
            \"turn\": 0,\
            \"turnkilled\": 0,\
            \"beamid\": 0,\
            \"engineid\": 0,\
            \"hullid\": 16,\
            \"ownerid\": 1,\
            \"torpedoid\": 0,\
            \"experience\": 0,\
            \"infoturn\": 7,\
            \"goal\": 0,\
            \"goaltarget\": 0,\
            \"goaltarget2\": 0,\
            \"waypoints\": [\
                {\
                    \"x\": 2626,\
                    \"y\": 1482\
                }\
            ],\
            \"history\": [\
                {\
                    \"x\": 2584,\
                    \"y\": 1544\
                },\
                {\
                    \"x\": 2543,\
                    \"y\": 1592\
                },\
                {\
                    \"x\": 2545,\
                    \"y\": 1635\
                },\
                {\
                    \"x\": 2543,\
                    \"y\": 1592\
                }\
            ],\
            \"iscloaked\": false,\
            \"readystatus\": 0,\
            \"id\": 1\
        }";
        let result = Ship {
            friendly_code: "".to_string(),
            name: "Medium Deep Space Freighter".to_string(),
            warp: 9i32,
            position: (1885i32, 1297i32),
            beams: 0i32,
            bays: 0i32,
            torps: 0i32,
            mission: 0i32,
            mission_1_target: 0i32,
            mission_2_target: 0i32,
            enemy: 0i32,
            damage: -1i32,
            crew: -1i32,
            clans: 0i32,
            neutronium: 0i32,
            tritanium: 0i32,
            duranium: 0i32,
            molybdenum: 0i32,
            supplies: 0i32,
            ammo: 0i32,
            megacredits: 0i32,
            transfer_clans: 0i32,
            transfer_neutronium: 0i32,
            transfer_duranium: 0i32,
            transfer_tritanium: 0i32,
            transfer_molybdenum: 0i32,
            transfer_supplies: 0i32,
            transfer_ammo: 0i32,
            transfer_megacredits: 0i32,
            transfer_target_id: 0i32,
            transfer_target_type: 0i32,
            target_position: (1885i32, 1300i32),
            mass: 212i32,
            heading: 0i32,
            turn: 0i32,
            turn_killed: 0i32,
            beam_id: 0i32,
            engine_id: 0i32,
            hull_id: 16i32,
            owner_id: 1i32,
            torpedo_id: 0i32,
            experience: 0i32,
            info_turn: 7i32,
            goal: 0i32,
            goal_target: 0i32,
            goal_target_2: 0i32,
            waypoints: vec![(2626i32, 1482i32)],
            history: vec![(2584i32, 1544i32), (2543i32, 1592i32), (2545i32, 1635i32), (2543i32, 1592i32)],
            is_cloaked: false,
            ready_status: 0i32,
            id: 1i32,
        };
        assert_eq!(result, build(&parse(json).unwrap()).unwrap());
    }
}
