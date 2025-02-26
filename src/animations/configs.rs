use bevy::prelude::*;

#[derive(Component)]
pub struct AnimateSprite {
    pub first: usize,
    pub last: usize,
    pub timer: Timer,
}