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

pub fn track_cursor_position(
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let window = windows.single();

    if let Some(cursor_position) = window.cursor_position() {
        if let Ok((camera, camera_transform)) = camera_query.get_single() {
            if let Ok(world_position) = camera.viewport_to_world(camera_transform, cursor_position)
            {
                let world_coords = world_position.origin.truncate(); // Vec2
                // info!("Cursor World Position: {:?}", world_coords);
            }
        }
    }
}

pub fn check_click_in_area(
    windows: Query<&Window>,
    buttons: Res<ButtonInput<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let window = windows.single();
    let range = 16.0;

    if let Some(cursor_position) = window.cursor_position() {
        if let Ok((camera, camera_transform)) = camera_query.get_single() {
            if let Ok(world_position) = camera.viewport_to_world(camera_transform, cursor_position)
            {
                let cursor_world_pos = world_position.origin.truncate(); // Vec2

                if cursor_world_pos.x >= SPAWN_X_LOCATION - range
                    && cursor_world_pos.x <= SPAWN_X_LOCATION + range
                    && cursor_world_pos.y >= SPAWN_Y_LOCATION - range
                    && cursor_world_pos.y <= SPAWN_Y_LOCATION + range
                {
                    if buttons.just_pressed(MouseButton::Left) {
                        info!("Clicked inside the area: {:?}", cursor_world_pos);
                    }
                }
            }
        }
    }
}
