extern crate serialize;

use self::serialize::json;

use builders::helpers;
use error;
use json_helpers::{find, get_bool, get_i64, get_list, get_object, get_string};

macro_rules! get(
    ($i1:ident, $e:expr, $i2:ident) => (try!($i2(try!(find($i1, $e)))))
)

// Public

#[deriving(Eq, PartialEq, Show)]
pub struct Ship {
    pub friendly_code: String,
    pub name: String,
    pub warp: i64,
    pub position: (i64, i64),
    pub beams: i64,
    pub bays: i64,
    pub torps: i64,
    pub mission: i64,
    pub mission_1_target: i64,
    pub mission_2_target: i64,
    pub enemy: i64,
    pub damage: i64,
    pub crew: i64,
    pub clans: i64,
    pub neutronium: i64,
    pub tritanium: i64,
    pub duranium: i64,
    pub molybdenum: i64,
    pub supplies: i64,
    pub ammo: i64,
    pub megacredits: i64,
    pub transfer_clans: i64,
    pub transfer_neutronium: i64,
    pub transfer_duranium: i64,
    pub transfer_tritanium: i64,
    pub transfer_molybdenum: i64,
    pub transfer_supplies: i64,
    pub transfer_ammo: i64,
    pub transfer_megacredits: i64,
    pub transfer_target_id: i64,
    pub transfer_target_type: i64,
    pub target_position: (i64, i64),
    pub mass: i64,
    pub heading: i64,
    pub turn: i64,
    pub turn_killed: i64,
    pub beam_id: i64,
    pub engine_id: i64,
    pub hull_id: i64,
    pub owner_id: i64,
    pub torpedo_id: i64,
    pub experience: i64,
    pub info_turn: i64,
    pub goal: i64,
    pub goal_target: i64,
    pub goal_target_2: i64,
    pub waypoints: Vec<(i64, i64)>,
    pub history: Vec<(i64, i64)>,
    pub is_cloaked: bool,
    pub ready_status: i64,
    pub id: i64,
}

pub fn build(json: &json::Json) -> Result<Ship, error::Error> {
    let map = try!(get_object(json));
    Ok(Ship {
        friendly_code: get!(map, "friendlycode", get_string),
        name: get!(map, "name", get_string),
        warp: get!(map, "warp", get_i64),
        position: (get!(map, "x", get_i64), get!(map, "y", get_i64)),
        beams: get!(map, "beams", get_i64),
        bays: get!(map, "bays", get_i64),
        torps: get!(map, "torps", get_i64),
        mission: get!(map, "mission", get_i64),
        mission_1_target: get!(map, "mission1target", get_i64),
        mission_2_target: get!(map, "mission2target", get_i64),
        enemy: get!(map, "enemy", get_i64),
        damage: get!(map, "damage", get_i64),
        crew: get!(map, "crew", get_i64),
        clans: get!(map, "clans", get_i64),
        neutronium: get!(map, "neutronium", get_i64),
        tritanium: get!(map, "tritanium", get_i64),
        duranium: get!(map, "duranium", get_i64),
        molybdenum: get!(map, "molybdenum", get_i64),
        supplies: get!(map, "supplies", get_i64),
        ammo: get!(map, "ammo", get_i64),
        megacredits: get!(map, "megacredits", get_i64),
        transfer_clans: get!(map, "transferclans", get_i64),
        transfer_neutronium: get!(map, "transferneutronium", get_i64),
        transfer_duranium: get!(map, "transferduranium", get_i64),
        transfer_tritanium: get!(map, "transfertritanium", get_i64),
        transfer_molybdenum: get!(map, "transfermolybdenum", get_i64),
        transfer_supplies: get!(map, "transfersupplies", get_i64),
        transfer_ammo: get!(map, "transferammo", get_i64),
        transfer_megacredits: get!(map, "transfermegacredits", get_i64),
        transfer_target_id: get!(map, "transfertargetid", get_i64),
        transfer_target_type: get!(map, "transfertargettype", get_i64),
        target_position: (get!(map, "targetx", get_i64), get!(map, "targety", get_i64)),
        mass: get!(map, "mass", get_i64),
        heading: get!(map, "heading", get_i64),
        turn: get!(map, "turn", get_i64),
        turn_killed: get!(map, "turnkilled", get_i64),
        beam_id: get!(map, "beamid", get_i64),
        engine_id: get!(map, "engineid", get_i64),
        hull_id: get!(map, "hullid", get_i64),
        owner_id: get!(map, "ownerid", get_i64),
        torpedo_id: get!(map, "torpedoid", get_i64),
        experience: get!(map, "experience", get_i64),
        info_turn: get!(map, "infoturn", get_i64),
        goal: get!(map, "goal", get_i64),
        goal_target: get!(map, "goaltarget", get_i64),
        goal_target_2: get!(map, "goaltarget2", get_i64),
        waypoints: try!(helpers::map_with_err(
            get!(map, "waypoints", get_list),
            |x| helpers::get_coordinates(x))),
        history: try!(helpers::map_with_err(
            get!(map, "history", get_list),
            |x| helpers::get_coordinates(x))),
        is_cloaked: get!(map, "iscloaked", get_bool),
        ready_status: get!(map, "readystatus", get_i64),
        id: get!(map, "id", get_i64),
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
            warp: 9i64,
            position: (1885i64, 1297i64),
            beams: 0i64,
            bays: 0i64,
            torps: 0i64,
            mission: 0i64,
            mission_1_target: 0i64,
            mission_2_target: 0i64,
            enemy: 0i64,
            damage: -1i64,
            crew: -1i64,
            clans: 0i64,
            neutronium: 0i64,
            tritanium: 0i64,
            duranium: 0i64,
            molybdenum: 0i64,
            supplies: 0i64,
            ammo: 0i64,
            megacredits: 0i64,
            transfer_clans: 0i64,
            transfer_neutronium: 0i64,
            transfer_duranium: 0i64,
            transfer_tritanium: 0i64,
            transfer_molybdenum: 0i64,
            transfer_supplies: 0i64,
            transfer_ammo: 0i64,
            transfer_megacredits: 0i64,
            transfer_target_id: 0i64,
            transfer_target_type: 0i64,
            target_position: (1885i64, 1300i64),
            mass: 212i64,
            heading: 0i64,
            turn: 0i64,
            turn_killed: 0i64,
            beam_id: 0i64,
            engine_id: 0i64,
            hull_id: 16i64,
            owner_id: 1i64,
            torpedo_id: 0i64,
            experience: 0i64,
            info_turn: 7i64,
            goal: 0i64,
            goal_target: 0i64,
            goal_target_2: 0i64,
            waypoints: vec![(2626i64, 1482i64)],
            history: vec![(2584i64, 1544i64), (2543i64, 1592i64), (2545i64, 1635i64), (2543i64, 1592i64)],
            is_cloaked: false,
            ready_status: 0i64,
            id: 1i64,
        };
        assert_eq!(result, build(&parse(json).unwrap()).unwrap());
    }
}
