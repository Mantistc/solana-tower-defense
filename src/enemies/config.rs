use bevy::prelude::*;

use super::{AnimateSprite, EnemyAnimation, EnemyAnimationState};

pub const TOTAL_WAVES: u8 = 30;
pub const MAX_ENEMIES_PER_WAVE: u8 = 30;
pub const SPAWN_Y_LOCATION: f32 = 150.0;
pub const SPAWN_X_LOCATION: f32 = 610.0;
pub const TIME_BETWEEN_WAVES: f32 = 5.0;
pub const TIME_BETWEEN_SPAWNS: f32 = 1.5;
pub const SCALE: f32 = 2.0;

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
    pub animations: Vec<EnemyAnimation>,
    pub spawned_count_in_wave: u8,
    pub time_between_waves: Timer,
}

pub fn load_enemy_sprites(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut textures: Vec<(Handle<Image>, Handle<TextureAtlasLayout>)> = Vec::new();
    let mut animations: Vec<EnemyAnimation> = Vec::new();

    let enemy_list = vec![
        (
            "enemies/orcs.png",
            UVec2::splat(48),
            8,
            6,
            EnemyAnimation {
                walk: AnimateSprite {
                    first: 8,
                    last: 15,
                    timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                },
                death: AnimateSprite {
                    first: 40,
                    last: 43,
                    timer: Timer::from_seconds(0.25, TimerMode::Repeating),
                },
                hurt: AnimateSprite {
                    first: 40,
                    last: 43,
                    timer: Timer::from_seconds(0.25, TimerMode::Repeating),
                },
                state: EnemyAnimationState::Walk,
            },
        ),
        (
            "enemies/soldier.png",
            UVec2::new(43, 31),
            7,
            6,
            EnemyAnimation {
                walk: AnimateSprite {
                    first: 0,
                    last: 6,
                    timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                },
                death: AnimateSprite {
                    first: 40,
                    last: 43,
                    timer: Timer::from_seconds(0.25, TimerMode::Repeating),
                },
                hurt: AnimateSprite {
                    first: 40,
                    last: 43,
                    timer: Timer::from_seconds(0.25, TimerMode::Repeating),
                },
                state: EnemyAnimationState::Walk,
            },
        ),
        (
            "enemies/Firebug.png",
            UVec2::new(128, 64),
            11,
            9,
            EnemyAnimation {
                walk: AnimateSprite {
                    first: 55,
                    last: 62,
                    timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                },
                death: AnimateSprite {
                    first: 55,
                    last: 62,
                    timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                },
                hurt: AnimateSprite {
                    first: 55,
                    last: 62,
                    timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                },
                state: EnemyAnimationState::Walk,
            },
        ),
    ];

    for (path, tile_size, columns, row, animation) in enemy_list {
        let texture = asset_server.load(path);
        let texture_atlas = TextureAtlasLayout::from_grid(tile_size, columns, row, None, None);
        let atlas_handle = texture_atlas_layouts.add(texture_atlas);

        textures.push((texture, atlas_handle));
        animations.push(animation);
    }

    commands.insert_resource(WaveControl {
        textures,
        animations,
        wave_count: 0,
        time_between_spawns: Timer::from_seconds(TIME_BETWEEN_SPAWNS, TimerMode::Repeating),
        spawned_count_in_wave: 0,
        time_between_waves: Timer::from_seconds(TIME_BETWEEN_WAVES, TimerMode::Once),
    });
    next_state.set(GameState::Playing);
}
