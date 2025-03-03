use bevy::prelude::*;

#[derive(Resource)]
pub struct Gold(u16);


pub const SPAWN_Y_LOCATION: f32 = 25.0;
pub const SPAWN_X_LOCATION: f32 = 15.0;
pub const TOWER_ATTACK_RANGE: f32 = 250.0;
