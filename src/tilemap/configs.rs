use bevy::prelude::Component;

pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 640.0;
pub const TILE_SIZE: f32 = 16.0;

#[derive(Component)]
pub struct Collider;

pub const COLLISION_THRESHOLD: f32 = 1750.0;
