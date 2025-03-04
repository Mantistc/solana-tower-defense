use bevy::prelude::*;

use super::{
    check_click_in_area, shot_enemies, spawn_shots_to_attack, spawn_tower, track_cursor_position,
};

#[derive(Resource, Debug)]
pub struct Gold(pub u16);

#[derive(Resource, Debug)]
pub struct Lifes(pub u8);

pub const SPAWN_Y_LOCATION: f32 = 25.0;
pub const SPAWN_X_LOCATION: f32 = 15.0;
pub const TOWER_ATTACK_RANGE: f32 = 250.0;
pub const DESPAWN_SHOT_RANGE: f32 = 800.0;
pub const SHOT_HURT_DISTANCE: f32 = 700.0;
pub const SHOT_SPEED: f32 = 700.0;

pub struct TowersPlugin;

impl Plugin for TowersPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gold(100))
            .insert_resource(Lifes(30))
            .add_systems(Startup, spawn_tower)
            // build systems
            .add_systems(Update, (track_cursor_position, check_click_in_area))
            // attack systems
            .add_systems(Update, (spawn_shots_to_attack, shot_enemies));
    }
}
