use bevy::prelude::*;

pub const TOTAL_WAVES: u8 = 30;
pub const MAX_ENEMIES_PER_WAVE: u8 = 50;
pub const SPAWN_Y_LOCATION: f32 = 150.0;
pub const SPAWN_X_LOCATION: f32 = 610.0;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Loading,
    Playing,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Loading
    }
}

#[derive(Resource, Debug)]
pub struct WaveControl {
    pub wave_count: u8,
    pub time_between_spawns: Timer,
    pub textures: Vec<(Handle<Image>, Handle<TextureAtlasLayout>)>,
    pub spawned_count_in_wave: u8
}

pub fn load_enemy_sprites(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut textures: Vec<(Handle<Image>, Handle<TextureAtlasLayout>)> = Vec::new();

    let enemy_list = vec!["enemies/orcs.png", "enemies/orcs.png", "enemies/orcs.png", "enemies/orcs.png"];

    for path in enemy_list {
        let texture = asset_server.load(path);
        let texture_atlas = TextureAtlasLayout::from_grid(UVec2::splat(48), 8, 6, None, None);
        let atlas_handle = texture_atlas_layouts.add(texture_atlas);

        textures.push((texture, atlas_handle));
    }

    commands.insert_resource(WaveControl {
        textures,
        wave_count: 0,
        time_between_spawns: Timer::from_seconds(1.5, TimerMode::Repeating),
        spawned_count_in_wave: 0
    });
    next_state.set(GameState::Playing);
    info!("loaded")
}
