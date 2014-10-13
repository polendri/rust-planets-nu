// Common structs

struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

// Login structs

struct PlayerSettings {
    player_planet_colors: (RGB, RGB),
    enemy_planet_colors: (RGB, RGB),
    ally_planet_colors: (RGB, RGB),
    info_planet_colors: (RGB, RGB),
    unknown_planet_colors: (RGB, RGB),
    player_ship_colors: (RGB, RGB),
    enemy_ship_colors: (RGB, RGB),
    ally_ship_colors: (RGB, RGB),
    player_mine_color: RGB,
    enemy_mine_color: RGB,
    web_mine_color: RGB,
    ally_mine_color: RGB,
    ion_storm_color: RGB,
    assistant_enabled: bool,
    mouse_zoom_enabled: bool,
    sound_effects_enabled: bool,
    music_enabled: bool,
    battle_task_id: i64,
    battle_tutorial_id: i64,
    id: i64,
}

struct LoginRequest<'a> {
    username: &'a str,
    password: &'a str,
}

struct LoginResult<'a> {
    api_key: &'a str,
    player_settings: PlayerSettings,
}

// Game Info structs

struct Game<'a> {
    name: &'a str,
    description: &'a str,
    short_description: &'a str,
    status: i64,
    created_datetime: &'a str, // datetime
    ended_datetime: &'a str, // datetime
    map_type: i64,
    game_type: i64,
    win_condition: i64,
    difficulty: f64,
    tutorial_id: i64,
    required_level_id: i64,
    max_level_id: i64,
    master_planet_id: i64,
    quadrant: i64,
    min_tenacity: i64,
    fast_start: i64,
    turns_per_week: i64,
    year_started: i64,
    is_private: bool,
    scenario_id: i64,
    created_by: &'a str,
    turn: i64,
    slots: i64,
    turn_status: &'a str, // its own datatype?
    host_days: &'a str, // its own datatype?
    slow_host_days: &'a str, // its own datatype?
    host_time: &'a str, // time
    last_backup_path: &'a str,
    next_host_datetime: &'a str, // datetime
    all_turns_in: bool,
    last_notified: bool,
    is_hosting: bool,
    last_loaded_datetime: &'a str, // datetime
    deleted_datetime: &'a str, // datetime
    last_host_datetime: &'a str, // datetime
    password: &'a str,
    has_password: bool,
    status_name: &'a str,
    just_ended: bool,
    time_to_host: &'a str,
    id: i64,
}

struct GameSettings<'a> {
    name: &'a str,
    turn: i64,
    build_queue_planet_id: i64,
    victory_countdown: i64,
    max_allies: i64,
    map_width: i64,
    map_height: i64,
    num_planets: i64,
    ship_limit: i64,
    host_start_datetime: &'a str,
    host_completed_datetime: &'a str,
    next_host_datetime: &'a str,
    last_invite_datetime: &'a str,
    team_size: i64,
    planet_scan_range: i64,
    ship_scan_range: i64,
    all_visible: bool,
    minefields_visible: bool,
    nebulas: i64,
    stars: i64,
    discussion_id: &'a str,
    nu_ion_storms: bool,
    max_ion_storms: i64,
    max_ion_clouds_per_storm: i64,
    debris_disk_percent: i64,
    debris_disk_version: i64,
    cloak_fail: i64,
    structure_decay_rate: i64,
    map_shape: i64,
    very_close_planets: i64,
    close_planets: i64,
    other_planets_min_homeworld_dist: i64,
    n_circles: i64, // ?
    hw_distribution: i64, // ?
    n_debris_discs: i64, // ?
    level_id: i64,
    next_level_id: i64,
    kill_race: bool,
    running_start: i64,
    dead_radius: i64,
    player_select_race: bool,
    military_score_percent: i64,
    hide_race_selection: bool,
    fixed_start_positions: bool,
    min_native_clans: i64,
    max_native_clans: i64,
    homeworld_has_starbase: bool,
    homeworld_clans: i64,
    homeworld_resources: i64,
    game_password: &'a str,
    neutronium_level: f64,
    duranium_level: f64,
    tritanium_level: f64,
    molybdenum_level: f64,
    average_density_percent: i64,
    development_factor: i64,
    native_probability: i64,
    native_government_level: i64,
    max_surface_neutronium: i64,
    max_surface_duranium: i64,
    max_surface_tritanium: i64,
    max_surface_molybdenum: i64,
    max_ground_neutronium: i64,
    max_ground_duranium: i64,
    max_ground_tritanium: i64,
    max_ground_molybdenum: i64,
    computer_build_ships: bool,
    computer_build_delay: i64,
    fight_or_fail: i64,
    show_all_explosions: bool,
    campaign_mode: bool,
    max_advantage: i64,
    fascist_double_beams: bool,
    production_queue: bool,
    production_base_cost: i64,
    production_starbase_output: i64,
    production_starbase_reward: i64,
    end_turn: i64,
    id: i64,
}

struct PlayerScore<'a> {
    added_datetime: &'a str,
    owner_id: i64,
    account_id: i64,
    capital_ships: i64,
    freighters: i64,
    planets: i64,
    starbases: i64,
    military_score: i64,
    inventory_score: i64,
    priority_points: i64,
    turn: i64,
    percent: f64,
    id: i64,
}

struct PlayerInfo<'a> {
    status: i64,
    status_turn: i64,
    account_id: i64,
    username: &'a str,
    email: &'a str,
    race_id: i64,
    team_id: i64,
    priority_points: i64,
    join_rank: i64,
    finish_rank: i64,
    turn_joined: i64,
    turn_ready: i64,
    turn_ready_datetime: &'a str,
    turn_status: i64,
    turns_missed: i64,
    turns_missed_total: i64,
    turns_holiday: i64,
    turns_early: i64,
    active_hulls: &'a [i64],
    active_advantages: &'a [i64],
    save_key: &'a str,
    tutorial_id: i64,
    tutorial_task_id: i64,
    is_registered: bool,
    level_id: i64,
    level_hull_id: i64,
    player_score: PlayerScore<'a>,
    planets: i64,
    ships: i64,
    starbases: i64,
    id: i64,
}

struct Relation {
    player_id: i64,
    player_to_id: i64,
    relation_to: i64,
    relation_from: i64,
    conflict_level: i64,
    color: RGB,
    id: i64,
}

struct GameInfo<'a> {
    game: Game<'a>,
    year_from: i64,
    year_to: Option<i64>,
    win_condition: &'a str,
    schedule: &'a str,
    time_to_host: &'a str,
    has_password: bool,
    settings: GameSettings<'a>,
    player_infos: &'a [PlayerInfo<'a>],
    relations: &'a [Relation],
}


#[test]
fn dummy_test() {
    assert!(1i == 2i);
}
