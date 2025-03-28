//! To build a **tower**, we first need to define its cost, damage per second (DPS), and other stats.
//! Each tower also has a **position** on the tilemap, meaning you can only place one tower per tile (obviously).
//!
//! This file contains all the constants and resources needed for the attack and building systems.

use super::*;
use bevy::{prelude::*, utils::hashbrown::HashMap};

pub struct TowersPlugin;

impl Plugin for TowersPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .insert_resource(Gold(INITIAL_PLAYER_GOLD))
            .insert_resource(Lifes(MAX_LIFES))
            .insert_resource(SelectedTowerType(TowerType::Lich))
            .add_systems(Startup, load_towers_sprites)
            .add_systems(
                OnEnter(GameState::GameOver),
                despawn_towers_and_reset_on_game_over,
            )
            // build systems
            .add_systems(
                Update,
                ((
                    select_tower_type,
                    setup_tower_zones,
                    buy_and_spawn_tower,
                    upgrade_tower,
                )
                    .run_if(in_state(GameState::Building)),),
            )
            .add_systems(
                OnEnter(GameState::Attacking),
                reset_hover_color_in_attacking,
            )
            // attack systems
            .add_systems(
                Update,
                (
                    spawn_shots,
                    move_shots_to_enemies,
                    despawn_shots_with_killed_target,
                )
                    .run_if(in_state(GameState::Attacking)),
            )
            .add_systems(OnEnter(GameState::Building), delete_all_shots_on_building);
    }
}

pub const COST_TABLE: [u16; 3] = [40, 100, 180];
pub const INITIAL_TOWER_DAMAGE: [u16; 3] = [15, 40, 150];
pub const TOWER_ATTACK_RANGE: f32 = 250.0;
pub const DESPAWN_SHOT_RANGE: f32 = 1500.0;
pub const SHOT_HURT_DISTANCE: f32 = 700.0;
pub const SHOT_SPEED: f32 = 700.0;
pub const SCALAR: f32 = 0.7;
pub const INITIAL_PLAYER_GOLD: u16 = 95;
pub const MAX_LIFES: u8 = 30;

pub const TOWER_POSITION_PLACEMENT: [Vec2; 15] = [
    Vec2::new(17.0, -64.0),
    Vec2::new(-112.0, -64.0),
    Vec2::new(144.0, -64.0),
    Vec2::new(-206.0, 190.0),
    Vec2::new(-335.0, 190.0),
    Vec2::new(-464.0, 190.0),
    Vec2::new(-240.0, -320.0),
    Vec2::new(-112.0, -320.0),
    Vec2::new(17.0, -320.0),
    Vec2::new(144.5, -320.0),
    Vec2::new(272.5, -320.0),
    Vec2::new(400.0, -27.0),
    Vec2::new(560.0, -27.0),
    Vec2::new(400.0, 190.0),
    Vec2::new(560.0, 190.0),
];

#[derive(States, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Building,
    Attacking,
    GameOver,
    Start,
    HowToPlay,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Start
    }
}

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct Gold(pub u16);

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct Lifes(pub u8);

/// Manages tower placement, textures, and valid build zones.
#[derive(Resource, Debug)]
pub struct TowerControl {
    /// Keeps track of which spots already have a tower placed
    pub placements: [u8; TOWER_POSITION_PLACEMENT.len()],
    /// Stores preloaded tower images for each level, so we can use them when spawning or upgrading towers
    pub textures: HashMap<(TowerType, u8), Handle<Image>>,
    /// Tower shots images and texture atlas based on the tower type
    pub shot_textures: HashMap<TowerType, (Handle<Image>, Handle<TextureAtlasLayout>)>,
    /// Holds entities representing valid tower placement zones, helping to check where towers can be built
    pub zones: Vec<Entity>,
}

/// Represents the different tower types available in the game.
/// Each tower type has three upgrade levels.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TowerType {
    Lich,
    Zigurat,
    Necro,
}

#[derive(Resource, Debug, Deref, DerefMut, Hash)]
pub struct SelectedTowerType(pub TowerType);

impl TowerType {
    /// Returns the cost of a tower based on its type and level
    /// The base cost is defined per tower type, and the price increases exponentially with level
    pub fn to_cost(&self, level: u8) -> u16 {
        let base_cost = match self {
            TowerType::Lich => COST_TABLE[0],
            TowerType::Zigurat => COST_TABLE[1],
            TowerType::Necro => COST_TABLE[2],
        };
        if level == 1 {
            return base_cost;
        }
        (base_cost as f32 * 1.3f32.powf(level as f32)).round() as u16
    }

    /// Generates the stats for a tower based on its type and level
    /// Includes attack damage and attack speed, both of which scale with level
    pub fn to_tower_data(&self, level: u8) -> TowerInfo {
        let base_damage = match self {
            TowerType::Lich => INITIAL_TOWER_DAMAGE[0],
            TowerType::Zigurat => INITIAL_TOWER_DAMAGE[1],
            TowerType::Necro => INITIAL_TOWER_DAMAGE[2],
        };

        // damage scales exponentially with level
        let attack_damage = ((base_damage as f32) * (1.1 + SCALAR).powf(level as f32))
            .round()
            .clamp(1.0, 500.0) as u16;

        let base_attack_speed = match self {
            TowerType::Lich => 0.5,
            TowerType::Zigurat => 0.4,
            TowerType::Necro => 1.2,
        };

        // attack speed scales with level, but has a minimum cap to prevent extreme speeds
        let attack_speed = Timer::from_seconds(
            (base_attack_speed * 0.85f32.powf(level as f32)).max(0.1),
            TimerMode::Repeating,
        );

        TowerInfo {
            attack_speed,
            attack_damage,
            level,
            tower_type: self.clone(),
        }
    }
}

/// Loads tower sprites and stores them in a hashmap for quick access when spawning or upgrading towers
pub fn load_towers_sprites(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut textures = HashMap::new();
    let mut shot_textures = HashMap::new();

    // TODO: i need to draw the next lvl sprites of this towers xdd
    let tower_sprites = vec![
        ((TowerType::Lich, 1), "towers/lich_01_tower.png"),
        ((TowerType::Lich, 2), "towers/lich_01_tower.png"),
        ((TowerType::Lich, 3), "towers/lich_01_tower.png"),
        ((TowerType::Zigurat, 1), "towers/zigurat_01_tower.png"),
        ((TowerType::Zigurat, 2), "towers/zigurat_01_tower.png"),
        ((TowerType::Zigurat, 3), "towers/zigurat_01_tower.png"),
        ((TowerType::Necro, 1), "towers/necro_01_tower.png"),
        ((TowerType::Necro, 2), "towers/necro_01_tower.png"),
        ((TowerType::Necro, 3), "towers/necro_01_tower.png"),
    ];

    let tower_shots = vec![
        (TowerType::Lich, "towers/shot_lich_tower.png"),
        (TowerType::Zigurat, "towers/shot_zigurat_tower.png"),
        (TowerType::Necro, "towers/shot_necro_tower.png"),
    ];

    for (tower_type, shot_path) in tower_shots {
        let texture = asset_server.load(shot_path);
        let texture_atlas = TextureAtlasLayout::from_grid(UVec2::splat(32), 8, 1, None, None);
        let atlas_handle = texture_atlas_layouts.add(texture_atlas);
        shot_textures.insert(tower_type, (texture, atlas_handle));
    }

    for (tower, path) in tower_sprites {
        let texture = asset_server.load(path);
        textures.insert(tower, texture);
    }

    commands.insert_resource(TowerControl {
        textures,
        placements: [0; TOWER_POSITION_PLACEMENT.len()],
        zones: [].to_vec(),
        shot_textures,
    });
}
