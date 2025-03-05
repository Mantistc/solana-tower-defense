use bevy::prelude::*;

use super::{TowerType, SPAWN_X_LOCATION, SPAWN_Y_LOCATION};

#[derive(Debug, Clone)]
pub struct TowerInfo {
    pub attack_damage: u8,
    pub attack_speed: Timer,
    pub level: u8,
    pub tower_type: TowerType,
}

#[derive(Component, Debug, Deref, DerefMut)]
pub struct Tower(pub TowerInfo);


impl Default for Tower {
    fn default() -> Self {
        Self(TowerInfo {
            attack_speed: Timer::from_seconds(0.25, TimerMode::Repeating),
            attack_damage: 5,
            level: 1,
            tower_type: TowerType::Lich,
        })
    }
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

pub fn click_and_spawn(
    windows: Query<&Window>,
    buttons: Res<ButtonInput<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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
                        let texture = asset_server.load("towers/tower.png");
                        commands.spawn((
                            Sprite::from_image(texture.clone()),
                            Tower::default(),
                            Transform {
                                translation: Vec3::new(SPAWN_X_LOCATION, SPAWN_Y_LOCATION, 1.0),
                                scale: Vec3::splat(2.0),
                                ..default()
                            },
                        ));
                    }
                }
            }
        }
    }
}
