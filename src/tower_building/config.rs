use std::{ops::Deref, u8};

use bevy::prelude::*;

use super::{
    click_and_spawn, shot_enemies, spawn_shots_to_attack, track_cursor_position, TowerInfo,
};

pub const SPAWN_Y_LOCATION: f32 = 25.0;
pub const SPAWN_X_LOCATION: f32 = 15.0;
pub const TOWER_ATTACK_RANGE: f32 = 250.0;
pub const DESPAWN_SHOT_RANGE: f32 = 800.0;
pub const SHOT_HURT_DISTANCE: f32 = 700.0;
pub const SHOT_SPEED: f32 = 700.0;
pub const CONTROL: f32 = 0.5;

pub struct TowersPlugin;

impl Plugin for TowersPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gold(100))
            .insert_resource(Lifes(30))
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
    pub textures: Vec<(Handle<Image>, Handle<TextureAtlasLayout>)>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TowerType {
    Lich,
    Zigurat,
    Electric,
}

impl TowerType {
    pub fn to_cost(&self, lvl: u8) -> u8 {
        const COST_TABLE: [[u8; 3]; 3] = [[5, 25, 50], [25, 75, 160], [50, 125, 225]];

        let tower_index = match self {
            TowerType::Lich => 0,
            TowerType::Zigurat => 1,
            TowerType::Electric => 2,
        };

        COST_TABLE[tower_index]
            .get((lvl - 1) as usize)
            .copied()
            .unwrap_or(u8::MAX)
    }

    pub fn to_tower_data(&self, lvl: u8) -> TowerInfo {
        let attack_speed = Timer::from_seconds(0.25, TimerMode::Repeating);

        let attack_damage = match self {
            TowerType::Lich => ((5 * lvl) as f32 * CONTROL).round().clamp(0.0, 255.0) as u8,
            TowerType::Zigurat => ((8 * lvl) as f32 * CONTROL).round().clamp(0.0, 255.0) as u8,
            TowerType::Electric => ((12 * lvl) as f32 * CONTROL).round().clamp(0.0, 255.0) as u8,
        };

        TowerInfo {
            attack_speed,
            attack_damage,
            level: lvl,
            tower_type: self.clone(),
        }
    }
}
pub const TOWER_POSITION_PLACEMENT: [Vec2; 13] = [
    Vec2::new(25.0, 15.0),
    Vec2::new(25.0, 15.0),
    Vec2::new(25.0, 15.0),
    Vec2::new(25.0, 15.0),
    Vec2::new(25.0, 15.0),
    Vec2::new(25.0, 15.0),
    Vec2::new(25.0, 15.0),
    Vec2::new(25.0, 15.0),
    Vec2::new(25.0, 15.0),
    Vec2::new(25.0, 15.0),
    Vec2::new(25.0, 15.0),
    Vec2::new(25.0, 15.0),
    Vec2::new(25.0, 15.0),
];
