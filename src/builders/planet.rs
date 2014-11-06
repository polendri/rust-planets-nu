extern crate serialize;

use self::serialize::json;

use error;
use json_helpers::{find, get_bool, get_i32, get_object, get_string};

// Public

#[deriving(Eq, PartialEq, Show)]
pub struct PlanetMineral {
    pub surface_count: i32,
    pub ground_count: i32,
    pub check_count: i32,
    pub density: i32,
    pub total: i32,
}

#[deriving(Eq, PartialEq, Show)]
pub struct Planet {
    pub name: String,
    pub position: (i32, i32),
    pub friendly_code: String,
    pub mines: i32,
    pub factories: i32,
    pub defense: i32,
    pub target_mines: i32,
    pub target_factories: i32,
    pub target_defense: i32,
    pub built_mines: i32,
    pub built_factories: i32,
    pub built_defense: i32,
    pub building_starbase: bool,
    pub megacredits: i32,
    pub supplies: i32,
    pub supplies_sold: i32,
    pub neutronium: PlanetMineral,
    pub molybdenum: PlanetMineral,
    pub duranium: PlanetMineral,
    pub tritanium: PlanetMineral,
    pub temp: i32,
    pub owner_id: i32,
    pub clans: i32,
    pub colonist_change: i32,
    pub colonist_tax_rate: i32,
    pub colonist_happy_points: i32,
    pub colonist_happy_change: i32,
    pub native_clans: i32,
    pub native_change: i32,
    pub native_government: i32,
    pub native_tax_value: i32,
    pub native_type: i32,
    pub native_tax_rate: i32,
    pub native_happy_points: i32,
    pub native_happy_change: i32,
    pub info_turn: i32,
    pub debris_disk: i32,
    pub flag: i32,
    pub ready_status: i32,
    pub image: String,
    pub native_race_name: String,
    pub native_government_name: String,
    pub id: i32,
}

pub fn build(json: &json::Json) -> Result<Planet, error::Error> {
    let map = try!(get_object(json));
    Ok(Planet {
        name:                   try!(get_string(try!(find(map, "name")))),
        position: (
            try!(get_i32(try!(find(map, "x")))),
            try!(get_i32(try!(find(map, "y"))))),
        friendly_code:          try!(get_string(try!(find(map, "friendlycode")))),
        mines:                  try!(get_i32(try!(find(map, "mines")))),
        factories:              try!(get_i32(try!(find(map, "factories")))),
        defense:                try!(get_i32(try!(find(map, "defense")))),
        target_mines:           try!(get_i32(try!(find(map, "targetmines")))),
        target_factories:       try!(get_i32(try!(find(map, "targetfactories")))),
        target_defense:         try!(get_i32(try!(find(map, "targetdefense")))),
        built_mines:            try!(get_i32(try!(find(map, "builtmines")))),
        built_factories:        try!(get_i32(try!(find(map, "builtfactories")))),
        built_defense:          try!(get_i32(try!(find(map, "builtdefense")))),
        building_starbase:      try!(get_bool(try!(find(map, "buildingstarbase")))),
        megacredits:            try!(get_i32(try!(find(map, "megacredits")))),
        supplies:               try!(get_i32(try!(find(map, "supplies")))),
        supplies_sold:          try!(get_i32(try!(find(map, "suppliessold")))),
        neutronium:
            PlanetMineral {
                surface_count:  try!(get_i32(try!(find(map, "neutronium")))),
                ground_count:   try!(get_i32(try!(find(map, "groundneutronium")))),
                check_count:    try!(get_i32(try!(find(map, "checkneutronium")))),
                density:        try!(get_i32(try!(find(map, "densityneutronium")))),
                total:          try!(get_i32(try!(find(map, "totalneutronium")))),
            },
        molybdenum:
            PlanetMineral {
                surface_count:  try!(get_i32(try!(find(map, "molybdenum")))),
                ground_count:   try!(get_i32(try!(find(map, "groundmolybdenum")))),
                check_count:    try!(get_i32(try!(find(map, "checkmolybdenum")))),
                density:        try!(get_i32(try!(find(map, "densitymolybdenum")))),
                total:          try!(get_i32(try!(find(map, "totalmolybdenum")))),
            },
        duranium:
            PlanetMineral {
                surface_count:  try!(get_i32(try!(find(map, "duranium")))),
                ground_count:   try!(get_i32(try!(find(map, "groundduranium")))),
                check_count:    try!(get_i32(try!(find(map, "checkduranium")))),
                density:        try!(get_i32(try!(find(map, "densityduranium")))),
                total:          try!(get_i32(try!(find(map, "totalduranium")))),
            },
        tritanium:
            PlanetMineral {
                surface_count:  try!(get_i32(try!(find(map, "tritanium")))),
                ground_count:   try!(get_i32(try!(find(map, "groundtritanium")))),
                check_count:    try!(get_i32(try!(find(map, "checktritanium")))),
                density:        try!(get_i32(try!(find(map, "densitytritanium")))),
                total:          try!(get_i32(try!(find(map, "totaltritanium")))),
            },
        temp:                   try!(get_i32(try!(find(map, "temp")))),
        owner_id:               try!(get_i32(try!(find(map, "ownerid")))),
        clans:                  try!(get_i32(try!(find(map, "clans")))),
        colonist_change:        try!(get_i32(try!(find(map, "colchange")))),
        colonist_tax_rate:      try!(get_i32(try!(find(map, "colonisttaxrate")))),
        colonist_happy_points:  try!(get_i32(try!(find(map, "colonisthappypoints")))),
        colonist_happy_change:  try!(get_i32(try!(find(map, "colhappychange")))),
        native_clans:           try!(get_i32(try!(find(map, "nativeclans")))),
        native_change:          try!(get_i32(try!(find(map, "nativechange")))),
        native_government:      try!(get_i32(try!(find(map, "nativegovernment")))),
        native_tax_value:       try!(get_i32(try!(find(map, "nativetaxvalue")))),
        native_type:            try!(get_i32(try!(find(map, "nativetype")))),
        native_tax_rate:        try!(get_i32(try!(find(map, "nativetaxrate")))),
        native_happy_points:    try!(get_i32(try!(find(map, "nativehappypoints")))),
        native_happy_change:    try!(get_i32(try!(find(map, "nativehappychange")))),
        info_turn:              try!(get_i32(try!(find(map, "infoturn")))),
        debris_disk:            try!(get_i32(try!(find(map, "debrisdisk")))),
        flag:                   try!(get_i32(try!(find(map, "flag")))),
        ready_status:           try!(get_i32(try!(find(map, "readystatus")))),
        image:                  try!(get_string(try!(find(map, "img")))),
        native_race_name:       try!(get_string(try!(find(map, "nativeracename")))),
        native_government_name: try!(get_string(try!(find(map, "nativegovernmentname")))),
        id:                     try!(get_i32(try!(find(map, "id")))),
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
            \"name\": \"Omicron 2 Eridani\",\
            \"x\": 2543,\
            \"y\": 1592,\
            \"friendlycode\": \"hGz\",\
            \"mines\": 364,\
            \"factories\": 265,\
            \"defense\": 20,\
            \"targetmines\": 0,\
            \"targetfactories\": 0,\
            \"targetdefense\": 0,\
            \"builtmines\": 4,\
            \"builtfactories\": 4,\
            \"builtdefense\": 0,\
            \"buildingstarbase\": false,\
            \"megacredits\": 23,\
            \"supplies\": 259,\
            \"suppliessold\": 0,\
            \"neutronium\": 158,\
            \"molybdenum\": 1851,\
            \"duranium\": 207,\
            \"tritanium\": 934,\
            \"groundneutronium\": 9011,\
            \"groundmolybdenum\": 5,\
            \"groundduranium\": 2827,\
            \"groundtritanium\": 1954,\
            \"densityneutronium\": 50,\
            \"densitymolybdenum\": 95,\
            \"densityduranium\": 15,\
            \"densitytritanium\": 20,\
            \"totalneutronium\": 0,\
            \"totalmolybdenum\": 0,\
            \"totalduranium\": 0,\
            \"totaltritanium\": 0,\
            \"checkneutronium\": 513,\
            \"checkmolybdenum\": 1921,\
            \"checkduranium\": 337,\
            \"checktritanium\": 1024,\
            \"checkmegacredits\": 13634,\
            \"checksupplies\": 1208,\
            \"temp\": 50,\
            \"ownerid\": 3,\
            \"clans\": 26448,\
            \"colchange\": 0,\
            \"colonisttaxrate\": 16,\
            \"colonisthappypoints\": 100,\
            \"colhappychange\": -6,\
            \"nativeclans\": 0,\
            \"nativechange\": 0,\
            \"nativegovernment\": 0,\
            \"nativetaxvalue\": 0,\
            \"nativetype\": 0,\
            \"nativetaxrate\": 0,\
            \"nativehappypoints\": 80,\
            \"nativehappychange\": 1,\
            \"infoturn\": 7,\
            \"debrisdisk\": 0,\
            \"flag\": 1,\
            \"readystatus\": 1,\
            \"img\": \"http://library.vgaplanets.nu/planets/150.png\",\
            \"nativeracename\": \"none\",\
            \"nativegovernmentname\": \"?\",\
            \"id\": 452\
        }";
        let result = Planet {
            name:                   "Omicron 2 Eridani".to_string(),
            position: (2543i32, 1592i32),
            friendly_code:          "hGz".to_string(),
            mines:                  364i32,
            factories:              265i32,
            defense:                20i32,
            target_mines:           0i32,
            target_factories:       0i32,
            target_defense:         0i32,
            built_mines:            4i32,
            built_factories:        4i32,
            built_defense:          0i32,
            building_starbase:      false,
            megacredits:            23i32,
            supplies:               259i32,
            supplies_sold:          0i32,
            neutronium:
                PlanetMineral {
                    surface_count:  158i32,
                    ground_count:   9011i32,
                    check_count:    513i32,
                    density:        50i32,
                    total:          0i32,
                },
            molybdenum:
                PlanetMineral {
                    surface_count:  1851i32,
                    ground_count:   5i32,
                    check_count:    1921i32,
                    density:        95i32,
                    total:          0i32,
                },
            duranium:
                PlanetMineral {
                    surface_count:  207i32,
                    ground_count:   2827i32,
                    check_count:    337i32,
                    density:        15i32,
                    total:          0i32,
                },
            tritanium:
                PlanetMineral {
                    surface_count:  934i32,
                    ground_count:   1954i32,
                    check_count:    1024i32,
                    density:        20i32,
                    total:          0i32,
                },
            temp:                   50i32,
            owner_id:               3i32,
            clans:                  26448i32,
            colonist_change:        0i32,
            colonist_tax_rate:      16i32,
            colonist_happy_points:  100i32,
            colonist_happy_change:  -6i32,
            native_clans:           0i32,
            native_change:          0i32,
            native_government:      0i32,
            native_tax_value:       0i32,
            native_type:            0i32,
            native_tax_rate:        0i32,
            native_happy_points:    80i32,
            native_happy_change:    1i32,
            info_turn:              7i32,
            debris_disk:            0i32,
            flag:                   1i32,
            ready_status:           1i32,
            image:                  "http://library.vgaplanets.nu/planets/150.png".to_string(),
            native_race_name:       "none".to_string(),
            native_government_name: "?".to_string(),
            id:                     452i32,
        };
        assert_eq!(result, build(&parse(json).unwrap()).unwrap());
    }
}
