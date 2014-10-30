extern crate serialize;

use self::serialize::json;

use builders::game::{mod, Game};
use builders::game_settings::{mod, GameSettings};
use builders::helpers;
use builders::minefield::{mod, Minefield};
use builders::planet::{mod, Planet};
use builders::player::{mod, Player};
use builders::player_score::{mod, PlayerScore};
use builders::player_settings::{mod, PlayerSettings};
use builders::ship::{mod, Ship};
use error;
use json_helpers::{find, get_bool, get_list, get_object, get_string};

macro_rules! get(
    ($i1:ident, $e:expr, $i2:ident) => (try!($i2(try!(find($i1, $e)))))
)

// Public

#[deriving(Eq, PartialEq, Show)]
pub struct LoadTurnResult {
    pub game_settings: GameSettings,
    pub game: Game,
    pub player: Player,
    pub players: Vec<Player>,
    pub player_scores: Vec<PlayerScore>,
    //pub maps: 
    pub planets: Vec<Planet>,
    pub ships: Vec<Ship>,
    //pub ion_storms: 
    //pub nebulas: 
    //pub stars: 
    //pub starbases: 
    //pub stock: 
    pub minefields: Vec<Minefield>,
    //pub relations: 
    //pub messages: 
    //pub my_messages: 
    //pub notes: 
    //pub vcrs: 
    //pub races: 
    //pub hulls: 
    //pub race_hulls: 
    //pub beams: 
    //pub engines: 
    //pub torpedos: 
    //pub advantages: 
    pub account_settings: PlayerSettings,
    pub save_key: String,
    pub is_premium: bool,
}

pub fn build(json: &json::Json) -> Result<LoadTurnResult, error::Error> {
    let map = try!(get_object(json));
    try!(helpers::check_success(map));
    let rst = try!(get_object(try!(find(map, "rst"))));
    Ok(LoadTurnResult {
        game_settings: try!(game_settings::build(try!(find(rst, "settings")))),
        game: try!(game::build(try!(find(rst, "game")))),
        player: try!(player::build(try!(find(rst, "player")))),
        players: try!(helpers::map_with_err(
            get!(rst, "players", get_list),
            |x| player::build(x))),
        player_scores: try!(helpers::map_with_err(
            get!(rst, "scores", get_list),
            |x| player_score::build(x))),
        //maps: 
        planets: try!(helpers::map_with_err(
            get!(rst, "planets", get_list),
            |x| planet::build(x))),
        ships: try!(helpers::map_with_err(
            get!(rst, "ships", get_list),
            |x| ship::build(x))),
        //ion_storms: 
        //nebulas: 
        //stars: 
        //starbases: 
        //stock: 
        minefields: try!(helpers::map_with_err(
            get!(rst, "minefields", get_list),
            |x| minefield::build(x))),
        //relations: 
        //messages: 
        //my_messages: 
        //notes: 
        //vcrs: 
        //races: 
        //hulls: 
        //race_hulls: 
        //beams: 
        //engines: 
        //torpedos: 
        //advantages: 
        account_settings: try!(player_settings::build(try!(find(map, "accountsettings")))),
        save_key: get!(map, "savekey", get_string),
        is_premium: get!(map, "ispremium", get_bool),
    })
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use builders::game::Game;
    use builders::game_settings::GameSettings;
    use builders::minefield::Minefield;
    use builders::planet::{Planet, PlanetMineral};
    use builders::player::Player;
    use builders::player_score::PlayerScore;
    use builders::player_settings::PlayerSettings;
    use builders::ship::Ship;
    use data::RGB;
    use json_helpers::parse;

    #[test]
    fn test_build() {
        let json = "{\
            \"success\": true,\
            \"rst\": {\
                \"settings\": {\
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
                },\
                \"game\": {\
                    \"name\":\"Vevalgoz Sector\",\
                    \"description\":\"This is a battle for the Vevalgoz Sector. This is a default configuration game. Custom map. This game has 2 turns per week.\",\
                    \"shortdescription\":\"\",\
                    \"status\":2,\
                    \"datecreated\":\"10\\/8\\/2011 12:04:45 PM\",\
                    \"dateended\":\"1\\/1\\/0001 12:00:00 AM\",\
                    \"maptype\":2,\
                    \"gametype\":2,\
                    \"wincondition\":1,\
                    \"difficulty\":0.881292261457551,\
                    \"tutorialid\":0,\
                    \"requiredlevelid\":0,\
                    \"maxlevelid\":0,\
                    \"masterplanetid\":257,\
                    \"quadrant\":20,\
                    \"mintenacity\":0,\
                    \"faststart\":0,\
                    \"turnsperweek\":2,\
                    \"yearstarted\":10,\
                    \"isprivate\":false,\
                    \"scenarioid\":0,\
                    \"createdby\":\"none\",\
                    \"turn\":327,\
                    \"slots\":11,\
                    \"turnstatus\":\"x2_x_x_x9_B\",\
                    \"hostdays\":\"S___T__\",\
                    \"slowhostdays\":\"\",\
                    \"hosttime\":\"2:27\",\
                    \"lastbackuppath\":\"c:\\\\planetsdata\\\\backups\\\\game25164\\\\turn326-635492683417604402.zip\",\
                    \"nexthost\":\"10\\/23\\/2014 2:27:00 AM\",\
                    \"allturnsin\":false,\
                    \"lastnotified\":false,\
                    \"ishosting\":false,\
                    \"lastloadeddate\":\"10\\/21\\/2014 8:12:55 PM\",\
                    \"deletedate\":\"\",\
                    \"lasthostdate\":\"10\\/18\\/2014 10:36:09 PM\",\
                    \"password\":\"\",\
                    \"haspassword\":false,\
                    \"statusname\":\"Running\",\
                    \"justended\":false,\
                    \"timetohost\":\"Next turn in 27 hours\",\
                    \"id\":25164\
                },\
                \"player\": {\
                    \"status\": 1,\
                    \"statusturn\": 1,\
                    \"accountid\": 12345,\
                    \"username\": \"that user name\",\
                    \"email\": \"\",\
                    \"raceid\": 9,\
                    \"teamid\": 0,\
                    \"prioritypoints\": 0,\
                    \"joinrank\": 0,\
                    \"finishrank\": 0,\
                    \"turnjoined\": 1,\
                    \"turnready\": true,\
                    \"turnreadydate\": \"10/8/2014 5:47:45 PM\",\
                    \"turnstatus\": 2,\
                    \"turnsmissed\": 0,\
                    \"turnsmissedtotal\": 0,\
                    \"turnsholiday\": 0,\
                    \"turnsearly\": 5,\
                    \"activehulls\": \"15,16,17,18,78,79,80,81,82,83,84,85,104,105\",\
                    \"activeadvantages\": \"24,25,45,48,49,51\",\
                    \"savekey\": \"\",\
                    \"tutorialid\": 0,\
                    \"tutorialtaskid\": 0,\
                    \"id\": 3\
                },\
                \"players\": [\
                    {\
                        \"status\": 1,\
                        \"statusturn\": 1,\
                        \"accountid\": 20001,\
                        \"username\": \"user 1\",\
                        \"email\": \"\",\
                        \"raceid\": 11,\
                        \"teamid\": 0,\
                        \"prioritypoints\": 0,\
                        \"joinrank\": 0,\
                        \"finishrank\": 0,\
                        \"turnjoined\": 1,\
                        \"turnready\": true,\
                        \"turnreadydate\": \"10/8/2014 5:32:27 PM\",\
                        \"turnstatus\": 2,\
                        \"turnsmissed\": 0,\
                        \"turnsmissedtotal\": 0,\
                        \"turnsholiday\": 0,\
                        \"turnsearly\": 5,\
                        \"activehulls\": \"14,15,16,17,18,42,86,89,92,95,96,97,98,99,100,101,102,103,104,105\",\
                        \"activeadvantages\": \"25,29,48,49,51\",\
                        \"savekey\": \"\",\
                        \"tutorialid\": 0,\
                        \"tutorialtaskid\": 0,\
                        \"id\": 1\
                    }\
                ],\
                \"scores\": [\
                    {\
                        \"dateadded\": \"10/8/2014 5:10:54 PM\",\
                        \"ownerid\": 1,\
                        \"accountid\": 12345,\
                        \"capitalships\": 5,\
                        \"freighters\": 2,\
                        \"planets\": 11,\
                        \"starbases\": 1,\
                        \"militaryscore\": 10754,\
                        \"inventoryscore\": 282,\
                        \"prioritypoints\": 0,\
                        \"turn\": 7,\
                        \"percent\": 17.88,\
                        \"id\": 37,\
                        \"shipchange\": 1,\
                        \"freighterchange\": 0,\
                        \"planetchange\": 4,\
                        \"starbasechange\": 0,\
                        \"militarychange\": 1099,\
                        \"inventorychange\": 50,\
                        \"prioritypointchange\": 0,\
                        \"percentchange\": -0.73\
                    }\
                ],\
                \"maps\": [],\
                \"planets\": [\
                    {\
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
                    }\
                ],\
                \"ships\": [\
                    {\
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
                    }\
                ],\
                \"ionstorms\": [\
                    {\
                        \"x\": 2427,\
                        \"y\": 2344,\
                        \"radius\": 136,\
                        \"voltage\": 41,\
                        \"warp\": 6,\
                        \"heading\": 173,\
                        \"isgrowing\": false,\
                        \"parentid\": 0,\
                        \"id\": 1\
                    }\
                ],\
                \"nebulas\": [],\
                \"stars\": [],\
                \"starbases\": [\
                    {\
                        \"defense\": 100,\
                        \"builtdefense\": 0,\
                        \"damage\": 0,\
                        \"enginetechlevel\": 10,\
                        \"hulltechlevel\": 6,\
                        \"beamtechlevel\": 3,\
                        \"torptechlevel\": 6,\
                        \"hulltechup\": 0,\
                        \"enginetechup\": 0,\
                        \"beamtechup\": 0,\
                        \"torptechup\": 0,\
                        \"fighters\": 20,\
                        \"builtfighters\": 0,\
                        \"shipmission\": 0,\
                        \"mission\": 0,\
                        \"planetid\": 452,\
                        \"raceid\": 0,\
                        \"targetshipid\": 0,\
                        \"buildbeamid\": 4,\
                        \"buildengineid\": 9,\
                        \"buildtorpedoid\": 0,\
                        \"buildhullid\": 84,\
                        \"buildbeamcount\": 2,\
                        \"buildtorpcount\": 0,\
                        \"isbuilding\": false,\
                        \"starbasetype\": 0,\
                        \"infoturn\": 7,\
                        \"readystatus\": 1,\
                        \"id\": 3\
                    }\
                ],\
                \"stock\": [\
                    {\
                        \"starbaseid\": 3,\
                        \"stocktype\": 1,\
                        \"stockid\": 15,\
                        \"amount\": 0,\
                        \"builtamount\": 0,\
                        \"id\": 114\
                    }\
                ],\
                \"minefields\": [\
                    {\
                        \"ownerid\":7,\
                        \"isweb\":true,\
                        \"units\":1253,\
                        \"infoturn\":17,\
                        \"friendlycode\":\"???\",\
                        \"x\":2729,\
                        \"y\":2335,\
                        \"radius\":35,\
                        \"id\":1\
                    }\
                ],\
                \"relations\": [\
                    {\
                        \"playerid\": 3,\
                        \"playertoid\": 1,\
                        \"relationto\": 0,\
                        \"relationfrom\": 3,\
                        \"conflictlevel\": 0,\
                        \"color\": \"\",\
                        \"id\": 13\
                    }\
                ],\
                \"messages\": [\
                    {\
                        \"ownerid\": 3,\
                        \"messagetype\": 5,\
                        \"headline\": \"Edna ID#255\",\
                        \"body\": \"The colonists on Edna ID#255 set up house keeping. We are now in control of the planet.\",\
                        \"target\": 255,\
                        \"turn\": 7,\
                        \"x\": 2537,\
                        \"y\": 1683,\
                        \"id\": 190\
                    }\
                ],\
                \"mymessages\": [\
                    {\
                        \"ownerid\": 6,\
                        \"messagetype\": 18,\
                        \"headline\": \"Diplomatic Council\",\
                        \"body\": \"An ambassador from The Robotic Imperium (rainbow stalin) has arrived! He has invited us to engage in open communication. \",\
                        \"target\": 3,\
                        \"turn\": 7,\
                        \"x\": 1,\
                        \"y\": 0,\
                        \"id\": 165\
                    }\
                ],\
                \"notes\": [\
                    {\
                        \"ownerid\": 3,\
                        \"body\": \"Economy: Industry\",\
                        \"targetid\": 456,\
                        \"targettype\": 1,\
                        \"color\": \"\",\
                        \"id\": 1\
                    }\
                ],\
                \"vcrs\": [],\
                \"races\": [\
                    {\
                        \"name\": \"Unknown\",\
                        \"shortname\": \"Unknown\",\
                        \"adjective\": \"Unknown\",\
                        \"baseadvantages\": \"\",\
                        \"advantages\": \"\",\
                        \"basehulls\": \"\",\
                        \"hulls\": \"\",\
                        \"id\": 0\
                    }\
                ],\
                \"hulls\": [\
                    {\
                        \"name\": \"Outrider Class Scout\",\
                        \"tritanium\": 40,\
                        \"duranium\": 20,\
                        \"molybdenum\": 5,\
                        \"fueltank\": 260,\
                        \"crew\": 180,\
                        \"engines\": 1,\
                        \"mass\": 75,\
                        \"techlevel\": 1,\
                        \"cargo\": 40,\
                        \"fighterbays\": 0,\
                        \"launchers\": 0,\
                        \"beams\": 1,\
                        \"cancloak\": false,\
                        \"cost\": 50,\
                        \"special\": \"\",\
                        \"description\": \"\",\
                        \"advantage\": 0,\
                        \"isbase\": true,\
                        \"dur\": 0,\
                        \"tri\": 0,\
                        \"mol\": 0,\
                        \"mc\": 0,\
                        \"parentid\": 0,\
                        \"id\": 1\
                    }\
                ],\
                \"racehulls\": [\
                    15,\
                    85\
                ],\
                \"beams\": [\
                    {\
                        \"name\": \"Laser\",\
                        \"cost\": 1,\
                        \"tritanium\": 1,\
                        \"duranium\": 0,\
                        \"molybdenum\": 0,\
                        \"mass\": 1,\
                        \"techlevel\": 1,\
                        \"crewkill\": 10,\
                        \"damage\": 3,\
                        \"id\": 1\
                    }\
                ],\
                \"engines\": [\
                    {\
                        \"name\": \"StarDrive 1\",\
                        \"cost\": 1,\
                        \"tritanium\": 5,\
                        \"duranium\": 1,\
                        \"molybdenum\": 0,\
                        \"techlevel\": 1,\
                        \"warp1\": 100,\
                        \"warp2\": 800,\
                        \"warp3\": 2700,\
                        \"warp4\": 6400,\
                        \"warp5\": 12500,\
                        \"warp6\": 21600,\
                        \"warp7\": 34300,\
                        \"warp8\": 51200,\
                        \"warp9\": 72900,\
                        \"id\": 1\
                    }\
                ],\
                \"torpedos\": [\
                    {\
                        \"name\": \"Mark 1 Photon\",\
                        \"torpedocost\": 1,\
                        \"launchercost\": 1,\
                        \"tritanium\": 1,\
                        \"duranium\": 1,\
                        \"molybdenum\": 0,\
                        \"mass\": 2,\
                        \"techlevel\": 1,\
                        \"crewkill\": 4,\
                        \"damage\": 5,\
                        \"id\": 1\
                    }\
                ],\
                \"advantages\": [\
                    {\
                        \"name\": \"Fed Crew Bonus\",\
                        \"description\": \"Fed ships will fight with an extra 50kt of hull mass, fighter ships will fight as if they have 3 extra fighter bays, damaged Fed ships will fight with all their weapons and 25% shields are recovered between fights. \",\
                        \"value\": 40,\
                        \"isbase\": true,\
                        \"locked\": true,\
                        \"dur\": 0,\
                        \"tri\": 0,\
                        \"mol\": 0,\
                        \"mc\": 0,\
                        \"id\": 1\
                    }\
                ]\
            },\
            \"accountsettings\": {\
                \"myplanetfrom\": \"#ccffff\",\
                \"myplanetto\": \"#00ffff\",\
                \"enemyplanetfrom\": \"#ffcccc\",\
                \"enemyplanetto\": \"#ff0000\",\
                \"allyplanetfrom\": \"#9999ff\",\
                \"allyplanetto\": \"#333399\",\
                \"infoplanetfrom\": \"#ffffcc\",\
                \"infoplanetto\": \"#993300\",\
                \"unknownplanetfrom\": \"#ffffff\",\
                \"unknownplanetto\": \"#505050\",\
                \"myshipfrom\": \"#00ff00\",\
                \"myshipto\": \"#009900\",\
                \"enemyshipfrom\": \"#ff0000\",\
                \"enemyshipto\": \"#990000\",\
                \"allyshipfrom\": \"#99ff99\",\
                \"allyshipto\": \"#669966\",\
                \"mymines\": \"#00ff00\",\
                \"enemymines\": \"#ff0000\",\
                \"webmines\": \"#f000f0\",\
                \"allymines\": \"#99ff99\",\
                \"ionstorms\": \"#ffff00\",\
                \"soundon\": \"true\",\
                \"musicon\": \"true\",\
                \"battletutorialid\": 1,\
                \"battletaskid\": 0,\
                \"assistanton\": true,\
                \"mousezoom\": true,\
                \"id\": 0\
            },\
            \"savekey\": \"\",\
            \"ispremium\": false\
        }";
        let result = LoadTurnResult {
            game_settings: GameSettings {
                name: "MFers Most Final Universe".to_string(),
                turn: 7i64,
                build_queue_planet_id: 0i64,
                victory_countdown: 0i64,
                max_allies: 2i64,
                map_width: 2000i64,
                map_height: 2000i64,
                num_planets: 500i64,
                ship_limit: 500i64,
                host_start_datetime: "10/8/2014 5:10:51 PM".to_string(),
                host_completed_datetime: "10/8/2014 5:10:59 PM".to_string(),
                next_host_datetime: "1/1/0001 12:00:00 AM".to_string(),
                last_invite_datetime: "1/1/0001 12:00:00 AM".to_string(),
                team_size: 0i64,
                planet_scan_range: 10000i64,
                ship_scan_range: 350i64,
                all_visible: false,
                minefields_visible: false,
                nebulas: 0i64,
                stars: 0i64,
                discussion_id: "".to_string(),
                nu_ion_storms: false,
                max_ion_storms: 4i64,
                max_ion_clouds_per_storm: 10i64,
                debris_disk_percent: 50i64,
                debris_disk_version: 2i64,
                cloak_fail: 0i64,
                structure_decay_rate: 3i64,
                map_shape: 0i64,
                very_close_planets: 4i64,
                close_planets: 15i64,
                other_planets_min_homeworld_dist: 155i64,
                n_circles: 1i64,
                hw_distribution: 2i64,
                n_debris_discs: 0i64,
                level_id: 0i64,
                next_level_id: 0i64,
                kill_race: false,
                running_start: 0i64,
                dead_radius: 81i64,
                player_select_race: false,
                military_score_percent: 65i64,
                hide_race_selection: false,
                fixed_start_positions: false,
                min_native_clans: 1000i64,
                max_native_clans: 75000i64,
                homeworld_has_starbase: true,
                homeworld_clans: 25000i64,
                homeworld_resources: 3i64,
                game_password: "".to_string(),
                neutronium_level: "2.08".to_string(),
                duranium_level: "1.25".to_string(),
                tritanium_level: "1.8".to_string(),
                molybdenum_level: "1.16".to_string(),
                average_density_percent: 55i64,
                development_factor: 1i64,
                native_probability: 55i64,
                native_government_level: 2i64,
                max_surface_neutronium: 250i64,
                max_surface_duranium: 40i64,
                max_surface_tritanium: 50i64,
                max_surface_molybdenum: 25i64,
                max_ground_neutronium: 700i64,
                max_ground_duranium: 500i64,
                max_ground_tritanium: 500i64,
                max_ground_molybdenum: 200i64,
                computer_build_ships: true,
                computer_build_delay: 0i64,
                fight_or_fail: 0i64,
                show_all_explosions: false,
                campaign_mode: false,
                max_advantage: 500i64,
                fascist_double_beams: false,
                production_queue: false,
                production_base_cost: 1i64,
                production_starbase_output: 2i64,
                production_starbase_reward: 2i64,
                end_turn: 100i64,
                id: 0i64,
            },
            game: Game {
                name: "Vevalgoz Sector".to_string(),
                description:
                    "This is a battle for the Vevalgoz Sector. This is a default configuration game. \
                    Custom map. This game has 2 turns per week.".to_string(),
                short_description: "".to_string(),
                status: 2i64,
                created_datetime: "10/8/2011 12:04:45 PM".to_string(),
                ended_datetime: "1/1/0001 12:00:00 AM".to_string(),
                map_type: 2i64,
                game_type: 2i64,
                win_condition: 1i64,
                difficulty: "0.881292261457551".to_string(),
                tutorial_id: 0i64,
                required_level_id: 0i64,
                max_level_id: 0i64,
                master_planet_id: 257i64,
                quadrant: 20i64,
                min_tenacity: 0i64,
                fast_start: 0i64,
                turns_per_week: 2i64,
                year_started: 10i64,
                is_private: false,
                scenario_id: 0i64,
                created_by: "none".to_string(),
                turn: 327i64,
                slots: 11i64,
                turn_status: "x2_x_x_x9_B".to_string(),
                host_days: "S___T__".to_string(),
                slow_host_days: "".to_string(),
                host_time: "2:27".to_string(),
                last_backup_path: "c:\\planetsdata\\backups\\game25164\\turn326-635492683417604402.zip".to_string(),
                next_host_datetime: "10/23/2014 2:27:00 AM".to_string(),
                all_turns_in: false,
                last_notified: false,
                is_hosting: false,
                last_loaded_datetime: "10/21/2014 8:12:55 PM".to_string(),
                deleted_datetime: "".to_string(),
                last_host_datetime: "10/18/2014 10:36:09 PM".to_string(),
                password: "".to_string(),
                has_password: false,
                status_name: "Running".to_string(),
                just_ended: false,
                time_to_host: "Next turn in 27 hours".to_string(),
                id: 25164i64,
            },
            player: Player {
                status: 1i64,
                status_turn: 1i64,
                account_id: 12345i64,
                username: "that user name".to_string(),
                email: "".to_string(),
                race_id: 9i64,
                team_id: 0i64,
                priority_points: 0i64,
                join_rank: 0i64,
                finish_rank: 0i64,
                turn_joined: 1i64,
                turn_ready: true,
                turn_ready_datetime: "10/8/2014 5:47:45 PM".to_string(),
                turn_status: 2i64,
                turns_missed: 0i64,
                turns_missed_total: 0i64,
                turns_holiday: 0i64,
                turns_early: 5i64,
                active_hulls: vec!["15","16","17","18","78","79","80","81","82","83","84","85","104","105"].iter().map(|x| x.to_string()).collect(),
                active_advantages: vec!["24","25","45","48","49","51"].iter().map(|x| x.to_string()).collect(),
                save_key: "".to_string(),
                tutorial_id: 0i64,
                tutorial_task_id: 0i64,
                id: 3i64,
            },
            players: vec![Player {
                status: 1i64,
                status_turn: 1i64,
                account_id: 20001i64,
                username: "user 1".to_string(),
                email: "".to_string(),
                race_id: 11i64,
                team_id: 0i64,
                priority_points: 0i64,
                join_rank: 0i64,
                finish_rank: 0i64,
                turn_joined: 1i64,
                turn_ready: true,
                turn_ready_datetime: "10/8/2014 5:32:27 PM".to_string(),
                turn_status: 2i64,
                turns_missed: 0i64,
                turns_missed_total: 0i64,
                turns_holiday: 0i64,
                turns_early: 5i64,
                active_hulls: vec!["14","15","16","17","18","42","86","89","92","95","96","97","98","99","100","101","102","103","104","105"].iter().map(|x| x.to_string()).collect(),
                active_advantages: vec!["25","29","48","49","51"].iter().map(|x| x.to_string()).collect(),
                save_key: "".to_string(),
                tutorial_id: 0i64,
                tutorial_task_id: 0i64,
                id: 1i64,
            }],
            player_scores: vec![PlayerScore {
                added_datetime: "10/8/2014 5:10:54 PM".to_string(),
                owner_id: 1i64,
                account_id: 12345i64,
                capital_ships: 5i64,
                freighters: 2i64,
                planets: 11i64,
                starbases: 1i64,
                military_score: 10754i64,
                inventory_score: 282i64,
                priority_points: 0i64,
                turn: 7i64,
                percent: "17.88".to_string(),
                id: 37i64,
            }],
            //maps: 
            planets: vec![Planet {
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
            }],
            ships: vec![Ship {
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
            }],
            //ion_storms: 
            //nebulas: 
            //stars: 
            //starbases: 
            //stock: 
            minefields: vec![Minefield {
                owner_id: 7i64,
                is_web: true,
                units: 1253i64,
                info_turn: 17i64,
                friendly_code: "???".to_string(),
                position: (2729i64, 2335i64),
                radius: 35i64,
                id: 1i64,
            }],
            //relations: 
            //messages: 
            //my_messages: 
            //notes: 
            //vcrs: 
            //races: 
            //hulls: 
            //race_hulls: 
            //beams: 
            //engines: 
            //torpedos: 
            //advantages: 
            account_settings: PlayerSettings {
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
            save_key: "".to_string(),
            is_premium: false,
        };
        assert_eq!(result, build(&parse(json).unwrap()).unwrap());
    }
}
