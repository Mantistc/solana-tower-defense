use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct AnimateSprite {
    pub first: usize,
    pub last: usize,
    pub timer: Timer,
}
