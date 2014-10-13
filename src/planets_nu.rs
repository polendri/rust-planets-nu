struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

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
    battle_task_id: int,
    battle_tutorial_id: int,
    id: int,
}

struct LoginRequest {
    username: &str,
    password: &str,
}

struct LoginResult {
    api_key: &str,
    player_settings: PlayerSettings,
}

pub fn dummy() {
}

#[test]
fn dummy_test() {
    assert!(1i == 2i);
}
