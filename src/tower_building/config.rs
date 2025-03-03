use bevy::prelude::*;

#[derive(Resource)]
pub struct Gold(u16);


pub const SPAWN_Y_LOCATION: f32 = 25.0;
pub const SPAWN_X_LOCATION: f32 = 15.0;
pub const TOWER_ATTACK_RANGE: f32 = 250.0;
pub const DESPAWN_SHOT_RANGE: f32 = 800.0;
pub const SHOT_HURT_DISTANCE: f32 = 700.0;
pub const SHOT_SPEED: f32 = 700.0;