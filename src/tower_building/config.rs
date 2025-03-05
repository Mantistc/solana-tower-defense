use std::{ops::Deref, u8};

use bevy::{prelude::*, utils::HashMap};

use crate::enemies::{AnimateSprite, EnemyAnimation, EnemyAnimationState};

use super::{
    click_and_spawn, shot_enemies, spawn_shots_to_attack, track_cursor_position, TowerInfo,
};

pub const TOWER_ATTACK_RANGE: f32 = 250.0;
pub const DESPAWN_SHOT_RANGE: f32 = 800.0;
pub const SHOT_HURT_DISTANCE: f32 = 700.0;
pub const SHOT_SPEED: f32 = 700.0;
pub const SCALAR: f32 = 0.5;

pub struct TowersPlugin;

impl Plugin for TowersPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gold(100))
            .insert_resource(Lifes(30))
            .add_systems(Startup, load_towers_sprites)
            // build systems
            .add_systems(Update, (track_cursor_position, click_and_spawn))
            // attack systems
            .add_systems(Update, (spawn_shots_to_attack, shot_enemies));
    }
}

#[derive(Resource, Debug)]
pub struct Gold(pub u16);

#[derive(Resource, Debug)]
pub struct Lifes(pub u8);

#[derive(Resource, Debug)]
pub struct TowerControl {
    // with this we can crontrol if in a specific position there is already a tower placed
    pub placements: [u8; TOWER_POSITION_PLACEMENT.len()],
    pub textures: HashMap<(TowerType, u8), (Handle<Image>, Handle<TextureAtlasLayout>)>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TowerType {
    Lich,
    Zigurat,
    Electric,
}

pub const COST_TABLE: [[u16; 3]; 3] = [[5, 25, 50], [25, 75, 160], [50, 125, 225]];
pub const INITIAL_TOWER_DAMAGE: [u8; 3] = [5, 10, 25];

impl TowerType {
    pub fn to_cost(&self, lvl: u8) -> u16 {
        let tower_index = match self {
            TowerType::Lich => 0,
            TowerType::Zigurat => 1,
            TowerType::Electric => 2,
        };

        COST_TABLE[tower_index]
            .get((lvl - 1) as usize)
            .copied()
            .unwrap_or(u16::MAX)
    }

    pub fn to_tower_data(&self, level: u8) -> TowerInfo {
        let attack_speed = Timer::from_seconds(0.25, TimerMode::Repeating);

        let attack_damage = match self {
            TowerType::Lich => ((INITIAL_TOWER_DAMAGE[0] * level) as f32 * SCALAR)
                .round()
                .clamp(0.0, 255.0) as u8,
            TowerType::Zigurat => ((INITIAL_TOWER_DAMAGE[1] * level) as f32 * SCALAR)
                .round()
                .clamp(0.0, 255.0) as u8,
            TowerType::Electric => ((INITIAL_TOWER_DAMAGE[2] * level) as f32 * SCALAR)
                .round()
                .clamp(0.0, 255.0) as u8,
        };

        TowerInfo {
            attack_speed,
            attack_damage,
            level,
            tower_type: self.clone(),
        }
    }
}
pub const TOWER_POSITION_PLACEMENT: [Vec2; 3] = [
    Vec2::new(25.0, 15.0),
    Vec2::new(-110.0, 15.0),
    Vec2::new(140.0, 15.0),
];

pub fn load_towers_sprites(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
) {
    let mut textures = HashMap::new();
    let mut animations: Vec<EnemyAnimation> = Vec::new();

    let enemy_list = vec![
        (
            (TowerType::Lich, 1),
            "towers/tower.png",
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
            (TowerType::Lich, 2),
            "towers/tower.png",
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
            (TowerType::Lich, 3),
            "towers/tower.png",
            UVec2::new(64, 64),
            8,
            9,
            EnemyAnimation {
                walk: AnimateSprite {
                    first: 40,
                    last: 47,
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

    for (tower, path, tile_size, columns, row, animation) in enemy_list {
        let texture = asset_server.load(path);
        let texture_atlas = TextureAtlasLayout::from_grid(tile_size, columns, row, None, None);
        let atlas_handle = texture_atlas_layouts.add(texture_atlas);

        textures.insert(tower, (texture, atlas_handle));
        animations.push(animation);
    }

    commands.insert_resource(TowerControl {
        textures,
        placements: [0; TOWER_POSITION_PLACEMENT.len()],
    });
}
