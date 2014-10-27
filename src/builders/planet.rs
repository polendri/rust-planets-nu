extern crate serialize;

use self::serialize::json;

use error;
use json_helpers::{find, get_bool, get_i64, get_object, get_string};

// Public

#[deriving(Eq, PartialEq, Show)]
pub struct PlanetMineral {
    pub surface_count: i64,
    pub ground_count: i64,
    pub check_count: i64,
    pub density: i64,
    pub total: i64,
}

#[deriving(Eq, PartialEq, Show)]
pub struct Planet {
    pub name: String,
    pub position: (i64, i64),
    pub friendly_code: String,
    pub mines: i64,
    pub factories: i64,
    pub defense: i64,
    pub target_mines: i64,
    pub target_factories: i64,
    pub target_defense: i64,
    pub built_mines: i64,
    pub built_factories: i64,
    pub built_defense: i64,
    pub building_starbase: bool,
    pub megacredits: i64,
    pub supplies: i64,
    pub supplies_sold: i64,
    pub neutronium: PlanetMineral,
    pub molybdenum: PlanetMineral,
    pub duranium: PlanetMineral,
    pub tritanium: PlanetMineral,
    pub temp: i64,
    pub owner_id: i64,
    pub clans: i64,
    pub colonist_change: i64,
    pub colonist_tax_rate: i64,
    pub colonist_happy_points: i64,
    pub colonist_happy_change: i64,
    pub native_clans: i64,
    pub native_change: i64,
    pub native_government: i64,
    pub native_tax_value: i64,
    pub native_type: i64,
    pub native_tax_rate: i64,
    pub native_happy_points: i64,
    pub native_happy_change: i64,
    pub info_turn: i64,
    pub debris_disk: i64,
    pub flag: i64,
    pub ready_status: i64,
    pub image: String,
    pub native_race_name: String,
    pub native_government_name: String,
    pub id: i64,
}

pub fn build(json: &json::Json) -> Result<Planet, error::Error> {
    let map = try!(get_object(json));
    Ok(Planet {
        name:                   try!(get_string(try!(find(map, "name")))),
        position: (
            try!(get_i64(try!(find(map, "x")))),
            try!(get_i64(try!(find(map, "y"))))),
        friendly_code:          try!(get_string(try!(find(map, "friendlycode")))),
        mines:                  try!(get_i64(try!(find(map, "mines")))),
        factories:              try!(get_i64(try!(find(map, "factories")))),
        defense:                try!(get_i64(try!(find(map, "defense")))),
        target_mines:           try!(get_i64(try!(find(map, "targetmines")))),
        target_factories:       try!(get_i64(try!(find(map, "targetfactories")))),
        target_defense:         try!(get_i64(try!(find(map, "targetdefense")))),
        built_mines:            try!(get_i64(try!(find(map, "builtmines")))),
        built_factories:        try!(get_i64(try!(find(map, "builtfactories")))),
        built_defense:          try!(get_i64(try!(find(map, "builtdefense")))),
        building_starbase:      try!(get_bool(try!(find(map, "buildingstarbase")))),
        megacredits:            try!(get_i64(try!(find(map, "megacredits")))),
        supplies:               try!(get_i64(try!(find(map, "supplies")))),
        supplies_sold:          try!(get_i64(try!(find(map, "suppliessold")))),
        neutronium:
            PlanetMineral {
                surface_count:  try!(get_i64(try!(find(map, "neutronium")))),
                ground_count:   try!(get_i64(try!(find(map, "groundneutronium")))),
                check_count:    try!(get_i64(try!(find(map, "checkneutronium")))),
                density:        try!(get_i64(try!(find(map, "densityneutronium")))),
                total:          try!(get_i64(try!(find(map, "totalneutronium")))),
            },
        molybdenum:
            PlanetMineral {
                surface_count:  try!(get_i64(try!(find(map, "molybdenum")))),
                ground_count:   try!(get_i64(try!(find(map, "groundmolybdenum")))),
                check_count:    try!(get_i64(try!(find(map, "checkmolybdenum")))),
                density:        try!(get_i64(try!(find(map, "densitymolybdenum")))),
                total:          try!(get_i64(try!(find(map, "totalmolybdenum")))),
            },
        duranium:
            PlanetMineral {
                surface_count:  try!(get_i64(try!(find(map, "duranium")))),
                ground_count:   try!(get_i64(try!(find(map, "groundduranium")))),
                check_count:    try!(get_i64(try!(find(map, "checkduranium")))),
                density:        try!(get_i64(try!(find(map, "densityduranium")))),
                total:          try!(get_i64(try!(find(map, "totalduranium")))),
            },
        tritanium:
            PlanetMineral {
                surface_count:  try!(get_i64(try!(find(map, "tritanium")))),
                ground_count:   try!(get_i64(try!(find(map, "groundtritanium")))),
                check_count:    try!(get_i64(try!(find(map, "checktritanium")))),
                density:        try!(get_i64(try!(find(map, "densitytritanium")))),
                total:          try!(get_i64(try!(find(map, "totaltritanium")))),
            },
        temp:                   try!(get_i64(try!(find(map, "temp")))),
        owner_id:               try!(get_i64(try!(find(map, "ownerid")))),
        clans:                  try!(get_i64(try!(find(map, "clans")))),
        colonist_change:        try!(get_i64(try!(find(map, "colchange")))),
        colonist_tax_rate:      try!(get_i64(try!(find(map, "colonisttaxrate")))),
        colonist_happy_points:  try!(get_i64(try!(find(map, "colonisthappypoints")))),
        colonist_happy_change:  try!(get_i64(try!(find(map, "colhappychange")))),
        native_clans:           try!(get_i64(try!(find(map, "nativeclans")))),
        native_change:          try!(get_i64(try!(find(map, "nativechange")))),
        native_government:      try!(get_i64(try!(find(map, "nativegovernment")))),
        native_tax_value:       try!(get_i64(try!(find(map, "nativetaxvalue")))),
        native_type:            try!(get_i64(try!(find(map, "nativetype")))),
        native_tax_rate:        try!(get_i64(try!(find(map, "nativetaxrate")))),
        native_happy_points:    try!(get_i64(try!(find(map, "nativehappypoints")))),
        native_happy_change:    try!(get_i64(try!(find(map, "nativehappychange")))),
        info_turn:              try!(get_i64(try!(find(map, "infoturn")))),
        debris_disk:            try!(get_i64(try!(find(map, "debrisdisk")))),
        flag:                   try!(get_i64(try!(find(map, "flag")))),
        ready_status:           try!(get_i64(try!(find(map, "readystatus")))),
        image:                  try!(get_string(try!(find(map, "img")))),
        native_race_name:       try!(get_string(try!(find(map, "nativeracename")))),
        native_government_name: try!(get_string(try!(find(map, "nativegovernmentname")))),
        id:                     try!(get_i64(try!(find(map, "id")))),
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
            position: (2543i64, 1592i64),
            friendly_code:          "hGz".to_string(),
            mines:                  364i64,
            factories:              265i64,
            defense:                20i64,
            target_mines:           0i64,
            target_factories:       0i64,
            target_defense:         0i64,
            built_mines:            4i64,
            built_factories:        4i64,
            built_defense:          0i64,
            building_starbase:      false,
            megacredits:            23i64,
            supplies:               259i64,
            supplies_sold:          0i64,
            neutronium:
                PlanetMineral {
                    surface_count:  158i64,
                    ground_count:   9011i64,
                    check_count:    513i64,
                    density:        50i64,
                    total:          0i64,
                },
            molybdenum:
                PlanetMineral {
                    surface_count:  1851i64,
                    ground_count:   5i64,
                    check_count:    1921i64,
                    density:        95i64,
                    total:          0i64,
                },
            duranium:
                PlanetMineral {
                    surface_count:  207i64,
                    ground_count:   2827i64,
                    check_count:    337i64,
                    density:        15i64,
                    total:          0i64,
                },
            tritanium:
                PlanetMineral {
                    surface_count:  934i64,
                    ground_count:   1954i64,
                    check_count:    1024i64,
                    density:        20i64,
                    total:          0i64,
                },
            temp:                   50i64,
            owner_id:               3i64,
            clans:                  26448i64,
            colonist_change:        0i64,
            colonist_tax_rate:      16i64,
            colonist_happy_points:  100i64,
            colonist_happy_change:  -6i64,
            native_clans:           0i64,
            native_change:          0i64,
            native_government:      0i64,
            native_tax_value:       0i64,
            native_type:            0i64,
            native_tax_rate:        0i64,
            native_happy_points:    80i64,
            native_happy_change:    1i64,
            info_turn:              7i64,
            debris_disk:            0i64,
            flag:                   1i64,
            ready_status:           1i64,
            image:                  "http://library.vgaplanets.nu/planets/150.png".to_string(),
            native_race_name:       "none".to_string(),
            native_government_name: "?".to_string(),
            id:                     452i64,
        };
        assert_eq!(result, build(&parse(json).unwrap()).unwrap());
    }
}
