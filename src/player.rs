use bevy::prelude::*;

use crate::tilemap::{Collider, COLLISION_THRESHOLD};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, PlayerPlugin::spawn)
            .add_systems(Update, (player_movement,animate_player, ));
    }
}

impl PlayerPlugin {
    pub fn spawn(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) {
        let texture = asset_server.load("player_sprite_sheet.png");
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 6, 6, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_indices = PlayerAnimation { first: 6, last: 11 };
        commands.spawn((
            Sprite::from_atlas_image(
                texture,
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
            ),
            PlayerAnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Player { speed: 250.0 },
            Transform {
                translation: Vec3::new(-55.0, -55.0, 1.0),
                scale: Vec3::splat(2.0),
                ..default()
            },
            animation_indices,
        ));
    }
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

#[derive(Component)]
pub struct PlayerAnimation {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct PlayerAnimationTimer(Timer);

fn animate_player(
    time: Res<Time>,
    mut query: Query<(&PlayerAnimation, &mut PlayerAnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ColliderDirection {
    Top,
    Bottom,
    Left,
    Right,
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
                ColliderDirection::Top,
                &colliders,
                movement_size,
            ) {
                transform.translation.y += movement_size;
            }
        }
        if input.pressed(KeyCode::KeyS) {
            if !collision(
                transform.translation,
                ColliderDirection::Bottom,
                &colliders,
                movement_size,
            ) {
                transform.translation.y -= movement_size;
            }
        }
        if input.pressed(KeyCode::KeyA) {
            if !collision(
                transform.translation,
                ColliderDirection::Left,
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
                ColliderDirection::Right,
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
    collider_direction: ColliderDirection,
    colliders: &Query<&Transform, With<Collider>>,
    movement_size: f32,
) -> bool {
    let new_position = match collider_direction {
        ColliderDirection::Top => player_translation + Vec3::new(0.0, movement_size, 0.0),
        ColliderDirection::Bottom => player_translation + Vec3::new(0.0, -movement_size, 0.0),
        ColliderDirection::Left => player_translation + Vec3::new(-movement_size, 0.0, 0.0),
        ColliderDirection::Right => player_translation + Vec3::new(movement_size, 0.0, 0.0),
    };

    for collider_transform in colliders.iter() {
        if new_position.distance_squared(collider_transform.translation) < COLLISION_THRESHOLD {
            return true;
        }
    }
    false
}
