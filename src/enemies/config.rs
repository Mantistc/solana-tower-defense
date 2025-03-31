//! Wave control determines how strong and fast enemies get with each wave.
//! Every wave is tougher than the last, so we need to **manage** how these values scale over time.
//!
//! This file handles that, so if you want enemies to attack faster, deal more damage, or take more hits,
//! this is where you make the changes.

use crate::tower_building::GameState;

use super::*;
use bevy::prelude::*;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_enemy_sprites)
            .add_systems(
                Update,
                (spawn_wave, animate, move_enemies, game_over)
                    .run_if(in_state(GameState::Attacking)),
            )
            .add_systems(
                Update,
                wave_control
                    .after(spawn_wave)
                    .run_if(in_state(GameState::Building).or(in_state(GameState::Attacking))),
            )
            .add_systems(
                OnEnter(GameState::GameOver),
                (
                    despawn_all_enemies_in_game_over,
                    reset_wave_control_on_game_over,
                )
                    .run_if(in_state(GameState::GameOver)),
            );
    }
}

pub const MAX_ENEMIES_PER_WAVE: u8 = 25;
pub const SPAWN_Y_LOCATION: f32 = 80.0;
pub const SPAWN_X_LOCATION: f32 = 610.0;
pub const TIME_BETWEEN_WAVES: f32 = 15.0;
pub const TIME_BETWEEN_SPAWNS: f32 = 1.5;
pub const INITIAL_ENEMY_LIFE: u16 = 60;
pub const SCALAR: f32 = 0.75;
pub const SCALE: f32 = 2.0;

/// Controls enemy waves, including spawn timing, textures, animations, and wave progression.
/// This resource is globally accessible to check and validate wave data.
#[derive(Resource, Debug)]
pub struct WaveControl {
    /// Current wave number.
    pub wave_count: u8,

    /// Timer controlling the interval between enemy spawns within a wave.
    pub time_between_spawns: Timer,

    /// List of enemy textures and their corresponding texture atlas layouts.
    pub textures: Vec<(Handle<Image>, Handle<TextureAtlasLayout>)>,

    /// Animations assigned to enemies.
    pub animations: Vec<EnemyAnimation>,

    /// Number of enemies spawned in the current wave.
    pub spawned_count_in_wave: u8,

    /// Timer controlling the interval between waves.
    pub time_between_waves: Timer,

    /// Value to control wether first wave needs to be spawned or not
    pub first_wave_spawned: bool,
}

pub fn ideal_time_per_frame() -> Timer {
    Timer::from_seconds(0.1, TimerMode::Repeating)
}

pub fn ideal_animation_values() -> EnemyAnimation {
    // this is the ideal values of the enemy sprite sheet that all enemy should have
    let standard_enemy_animation = EnemyAnimation {
        walk_up: AnimateSprite {
            first: 0,
            last: 3,
            ..default()
        },
        walk_left: AnimateSprite {
            first: 8,
            last: 11,
            ..default()
        },
        walk_down: AnimateSprite {
            first: 12,
            last: 15,
            ..default()
        },
        ..default()
    };
    standard_enemy_animation
}

pub fn load_enemy_sprites(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
) {
    let mut textures: Vec<(Handle<Image>, Handle<TextureAtlasLayout>)> = Vec::new();
    let mut animations: Vec<EnemyAnimation> = Vec::new();

    let enemy_list = get_enemy_list();

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
        first_wave_spawned: false,
    });
}
