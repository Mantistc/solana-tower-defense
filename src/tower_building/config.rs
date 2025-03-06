use bevy::{prelude::*, utils::HashMap};

use crate::enemies::{AnimateSprite, EnemyAnimation, EnemyAnimationState};

use super::{buy_tower, select_tower_type, shot_enemies, spawn_shots_to_attack, TowerInfo};

pub const TOWER_ATTACK_RANGE: f32 = 250.0;
pub const DESPAWN_SHOT_RANGE: f32 = 800.0;
pub const SHOT_HURT_DISTANCE: f32 = 700.0;
pub const SHOT_SPEED: f32 = 700.0;
pub const SCALAR: f32 = 0.5;

pub struct TowersPlugin;

impl Plugin for TowersPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gold(25))
            .insert_resource(Lifes(30))
            .insert_resource(SelectedTowerType(TowerType::Lich))
            .add_systems(Startup, load_towers_sprites)
            // build systems
            .add_systems(Update, (select_tower_type, buy_tower))
            // attack systems
            .add_systems(Update, (spawn_shots_to_attack, shot_enemies));
    }
}

#[derive(Resource, Debug, Deref, DerefMut)]
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

#[derive(Resource, Debug, Deref, DerefMut, Hash)]
pub struct SelectedTowerType(pub TowerType);

pub const COST_TABLE: [[u16; 3]; 3] = [[25, 100, 180], [75, 150, 300], [125, 350, 600]];
pub const INITIAL_TOWER_DAMAGE: [u8; 3] = [10, 15, 30];
pub const TOWER_POSITION_PLACEMENT: [Vec2; 14] = [
    Vec2::new(17.0, 15.0),
    Vec2::new(-110.0, 15.0),
    Vec2::new(140.0, 15.0),
    Vec2::new(-210.0, 270.0),
    Vec2::new(-340.0, 270.0),
    Vec2::new(-465.0, 270.0),
    Vec2::new(-335.0, 65.0),
    Vec2::new(-240.0, -230.0),
    Vec2::new(17.0, -230.0),
    Vec2::new(268.0, -230.0),
    Vec2::new(400.0, 53.0),
    Vec2::new(560.0, 53.0),
    Vec2::new(400.0, 270.0),
    Vec2::new(560.0, 270.0),
];

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

pub fn load_towers_sprites(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
) {
    let mut textures = HashMap::new();
    let mut animations: Vec<EnemyAnimation> = Vec::new();

    let tower_sprites = vec![
        (
            (TowerType::Lich, 1),
            "towers/lich_01_tower.png",
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
            "towers/lich_01_tower.png",
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
            "towers/lich_01_tower.png",
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
        (
            (TowerType::Zigurat, 1),
            "towers/zigurat_01_tower.png",
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
        (
            (TowerType::Zigurat, 2),
            "towers/zigurat_01_tower.png",
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
        (
            (TowerType::Zigurat, 3),
            "towers/zigurat_01_tower.png",
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
        (
            (TowerType::Electric, 1),
            "towers/electric_01_tower.png",
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
        (
            (TowerType::Electric, 2),
            "towers/electric_01_tower.png",
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
        (
            (TowerType::Electric, 3),
            "towers/electric_01_tower.png",
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

    for (tower, path, tile_size, columns, row, animation) in tower_sprites {
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
