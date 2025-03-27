//! Wave control determines how strong and fast enemies get with each wave.
//! Every wave is tougher than the last, so we need to **manage** how these values scale over time.
//!
//! This file handles that, so if you want enemies to attack faster, deal more damage, or take more hits,
//! this is where you make the changes.

use super::{AnimateSprite, EnemyAnimation, EnemyAnimationState};
use bevy::prelude::*;

pub const MAX_ENEMIES_PER_WAVE: u8 = 25;
pub const SPAWN_Y_LOCATION: f32 = 80.0;
pub const SPAWN_X_LOCATION: f32 = 610.0;
pub const TIME_BETWEEN_WAVES: f32 = 15.0;
pub const TIME_BETWEEN_SPAWNS: f32 = 1.5;
pub const INITIAL_ENEMY_LIFE: u16 = 60;
pub const SCALAR: f32 = 0.8;
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

fn ideal_time_per_frame() -> Timer {
    Timer::from_seconds(0.1, TimerMode::Repeating)
}

fn ideal_animation_values() -> EnemyAnimation {
    // this is the ideal values of the enemy sprite sheet that all enemy should have
    let standard_enemy_animation = EnemyAnimation {
        walk_up: AnimateSprite {
            first: 4,
            last: 7,
            timer: ideal_time_per_frame(),
        },
        walk_right: AnimateSprite {
            first: 4,
            last: 7,
            timer: ideal_time_per_frame(),
        },
        walk_left: AnimateSprite {
            first: 4,
            last: 7,
            timer: ideal_time_per_frame(),
        },
        walk_down: AnimateSprite {
            first: 4,
            last: 7,
            timer: ideal_time_per_frame(),
        },
        state: EnemyAnimationState::WalkLeft,
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

    let columns = 4;
    let rows = 4;
    let enemy_list = vec![
        (
            "enemies/orcs.png",
            UVec2::splat(48),
            8,
            6,
            EnemyAnimation {
                walk_up: AnimateSprite {
                    first: 8,
                    last: 15,
                    timer: ideal_time_per_frame(),
                },
                walk_left: AnimateSprite {
                    first: 8,
                    last: 15,
                    timer: ideal_time_per_frame(),
                },
                walk_down: AnimateSprite {
                    first: 8,
                    last: 15,
                    timer: ideal_time_per_frame(),
                },
                walk_right: AnimateSprite {
                    first: 8,
                    last: 15,
                    timer: ideal_time_per_frame(),
                },
                state: EnemyAnimationState::WalkLeft,
            },
        ),
        (
            "enemies/ohai.png",
            UVec2::splat(32),
            columns,
            rows,
            ideal_animation_values(),
        ),
        (
            "enemies/micuwa.png",
            UVec2::splat(32),
            columns,
            rows,
            ideal_animation_values(),
        ),

        (
            "enemies/soldier.png",
            UVec2::new(43, 31),
            7,
            6,
            EnemyAnimation {
                state: EnemyAnimationState::WalkLeft,
                walk_up: AnimateSprite {
                    first: 0,
                    last: 6,
                    timer: ideal_time_per_frame(),
                },
                walk_down: AnimateSprite {
                    first: 0,
                    last: 6,
                    timer: ideal_time_per_frame(),
                },
                walk_left: AnimateSprite {
                    first: 0,
                    last: 6,
                    timer: ideal_time_per_frame(),
                },
                walk_right: AnimateSprite {
                    first: 0,
                    last: 6,
                    timer: ideal_time_per_frame(),
                },
            },
        ),
        (
            "enemies/Leafbug.png",
            UVec2::new(64, 64),
            8,
            9,
            EnemyAnimation {
                state: EnemyAnimationState::WalkLeft,
                walk_up: AnimateSprite {
                    first: 40,
                    last: 47,
                    timer: ideal_time_per_frame(),
                },
                walk_down: AnimateSprite {
                    first: 40,
                    last: 47,
                    timer: ideal_time_per_frame(),
                },
                walk_left: AnimateSprite {
                    first: 40,
                    last: 47,
                    timer: ideal_time_per_frame(),
                },
                walk_right: AnimateSprite {
                    first: 40,
                    last: 47,
                    timer: ideal_time_per_frame(),
                },
            },
        ),
        (
            "enemies/Firebug.png",
            UVec2::new(128, 64),
            11,
            9,
            EnemyAnimation {
                state: EnemyAnimationState::WalkLeft,
                walk_up: AnimateSprite {
                    first: 55,
                    last: 62,
                    timer: ideal_time_per_frame(),
                },
                walk_down: AnimateSprite {
                    first: 40,
                    last: 47,
                    timer: ideal_time_per_frame(),
                },
                walk_left: AnimateSprite {
                    first: 40,
                    last: 47,
                    timer: ideal_time_per_frame(),
                },
                walk_right: AnimateSprite {
                    first: 40,
                    last: 47,
                    timer: ideal_time_per_frame(),
                },
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
        first_wave_spawned: false,
    });
}
