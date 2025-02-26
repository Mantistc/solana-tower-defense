use bevy::prelude::*;

use crate::tilemap::{Collider, COLLISION_THRESHOLD};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, PlayerPlugin::spawn).add_systems(
            Update,
            (
                player_movement,
                animate_player_idle,
                animate_player_movement,
            ),
        );
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
        let player_animation_running = PlayerRunningAnimation(AnimateSprite {
            first: 6,
            last: 11,
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        });

        let player_animation_idle = PlayerIdleAnimation(AnimateSprite {
            first: 0,
            last: 3,
            timer: Timer::from_seconds(0.35, TimerMode::Repeating),
        });
        commands.spawn((
            Sprite::from_atlas_image(
                texture,
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: player_animation_idle.first,
                },
            ),
            Player { speed: 250.0 },
            Transform {
                translation: Vec3::new(-55.0, -55.0, 1.0),
                scale: Vec3::splat(2.0),
                ..default()
            },
            player_animation_running,
            player_animation_idle,
        ));
    }
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

#[derive(Component)]
pub struct AnimateSprite {
    pub first: usize,
    pub last: usize,
    pub timer: Timer,
}

#[derive(Component, Deref, DerefMut)]
pub struct PlayerRunningAnimation(AnimateSprite);

#[derive(Component, Deref, DerefMut)]
pub struct PlayerIdleAnimation(AnimateSprite);

pub fn animate_player_movement(
    time: Res<Time>,
    mut player_animation_query: Query<(&mut PlayerRunningAnimation, &mut Sprite)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if let Ok((mut player_animation, mut sprite)) = player_animation_query.get_single_mut() {
        if input.pressed(KeyCode::KeyW)
            || input.pressed(KeyCode::KeyA)
            || input.pressed(KeyCode::KeyD)
            || input.pressed(KeyCode::KeyS)
        {
            player_animation.timer.tick(time.delta());

            if player_animation.timer.just_finished() {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = if atlas.index < 6
                        || atlas.index > 11
                        || atlas.index == player_animation.last
                    {
                        player_animation.first
                    } else if atlas.index < 11 && atlas.index >= 6 {
                        atlas.index + 1
                    } else {
                        atlas.index
                    }
                }
            }
        }
    }
}

pub fn animate_player_idle(
    time: Res<Time>,
    mut player_animation_query: Query<(&mut PlayerIdleAnimation, &mut Sprite)>,
) {
    if let Ok((mut player_animation, mut sprite)) = player_animation_query.get_single_mut() {
        player_animation.timer.tick(time.delta());

        if player_animation.timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index > 4 || atlas.index == player_animation.last {
                    player_animation.first
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
