use bevy::prelude::*;

use crate::{
    animations::{animate_player, PlayerAnimations},
    tilemap::{Collider, COLLISION_THRESHOLD},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (player_movement, animate_player));
    }
}
#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub life: u8,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 250.0,
            life: 100,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum MovementDirection {
    Top,
    Bottom,
    Left,
    Right,
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("player/player_sprite_sheet.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 6, 6, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animations = PlayerAnimations::default();

    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: player_animations.idle.first,
            },
        ),
        Player::default(),
        Transform {
            translation: Vec3::new(100.0, -150.0, 1.1),
            scale: Vec3::splat(2.0),
            ..default()
        },
        player_animations,
    ));
}

pub fn player_movement(
    mut players: Query<(&mut Transform, &Player), Without<Collider>>,
    colliders: Query<&Transform, With<Collider>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut players {
        let movement_size = (player.speed * time.delta_secs()).round();
        if input.pressed(KeyCode::KeyW) {
            if !collision(
                transform.translation,
                MovementDirection::Top,
                &colliders,
                movement_size,
            ) {
                transform.translation.y += movement_size;
            }
        }
        if input.pressed(KeyCode::KeyS) {
            if !collision(
                transform.translation,
                MovementDirection::Bottom,
                &colliders,
                movement_size,
            ) {
                transform.translation.y -= movement_size;
            }
        }
        if input.pressed(KeyCode::KeyA) {
            if !collision(
                transform.translation,
                MovementDirection::Left,
                &colliders,
                movement_size,
            ) {
                transform.translation.x -= movement_size;
            }
            transform.scale.x = -2.0;
        }
        if input.pressed(KeyCode::KeyD) {
            if !collision(
                transform.translation,
                MovementDirection::Right,
                &colliders,
                movement_size,
            ) {
                transform.translation.x += movement_size;
            }
            transform.scale.x = 2.0;
        }
    }
}

fn collision(
    player_translation: Vec3,
    collider_direction: MovementDirection,
    colliders: &Query<&Transform, With<Collider>>,
    movement_size: f32,
) -> bool {
    let new_position = match collider_direction {
        MovementDirection::Top => player_translation + Vec3::new(0.0, movement_size, 0.0),
        MovementDirection::Bottom => player_translation + Vec3::new(0.0, -movement_size, 0.0),
        MovementDirection::Left => player_translation + Vec3::new(-movement_size, 0.0, 0.0),
        MovementDirection::Right => player_translation + Vec3::new(movement_size, 0.0, 0.0),
    };

    for collider_transform in colliders.iter() {
        if new_position.distance_squared(collider_transform.translation) < COLLISION_THRESHOLD {
            return true;
        }
    }
    false
}
