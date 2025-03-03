use bevy::prelude::*;

use super::{SPAWN_X_LOCATION, SPAWN_Y_LOCATION};

#[derive(Component, Debug)]
pub struct Tower {
    pub attack_damage: u8,
    pub attack_speed: Timer,
}


impl Default for Tower {
    fn default() -> Self {
        Self {
            attack_speed: Timer::from_seconds(0.25, TimerMode::Repeating),
            attack_damage: 5,
        }
    }
}

pub fn spawn_tower(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("towers/tower.png"); // simulate this is a tower
    commands.spawn((
        Sprite::from_image(texture.clone()),
        Tower::default(),
        Transform {
            translation: Vec3::new(135.0, SPAWN_Y_LOCATION, 1.0),
            scale: Vec3::splat(2.0),
            ..default()
        },
    ));
    commands.spawn((
        Sprite::from_image(texture.clone()),
        Tower::default(),
        Transform {
            translation: Vec3::new(SPAWN_X_LOCATION, SPAWN_Y_LOCATION, 1.0),
            scale: Vec3::splat(2.0),
            ..default()
        },
    ));
    commands.spawn((
        Sprite::from_image(texture.clone()),
        Tower {
            attack_damage: 10,
            attack_speed: Timer::from_seconds(0.20, TimerMode::Repeating),
        },
        Transform {
            translation: Vec3::new(-110.0, SPAWN_Y_LOCATION, 1.0),
            scale: Vec3::splat(2.0),
            ..default()
        },
    ));
}