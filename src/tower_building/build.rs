use bevy::prelude::*;

use super::{Gold, TowerControl, TowerType, TOWER_POSITION_PLACEMENT};

#[derive(Debug, Clone)]
pub struct TowerInfo {
    pub attack_damage: u8,
    pub attack_speed: Timer,
    pub level: u8,
    pub tower_type: TowerType,
}

#[derive(Component, Debug, Deref, DerefMut)]
pub struct Tower(pub TowerInfo);


pub fn click_and_spawn(
    windows: Query<&Window>,
    buttons: Res<ButtonInput<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut commands: Commands,
    mut tower_control: ResMut<TowerControl>,
    mut gold: ResMut<Gold>,
) {
    let window = windows.single();
    let range = 32.0;

    if let Some(cursor_position) = window.cursor_position() {
        if let Ok((camera, camera_transform)) = camera_query.get_single() {
            if let Ok(world_position) = camera.viewport_to_world(camera_transform, cursor_position)
            {
                let cursor_world_pos = world_position.origin.truncate(); // Vec2
                for (i, placement) in TOWER_POSITION_PLACEMENT.iter().enumerate() {
                    if cursor_world_pos.x >= placement.x - range
                        && cursor_world_pos.x <= placement.x + range
                        && cursor_world_pos.y >= placement.y - range
                        && cursor_world_pos.y <= placement.y + range
                    {
                        if tower_control.placements[i] == 0
                            && buttons.just_pressed(MouseButton::Left)
                        {
                            let tower_type = TowerType::Lich;
                            let tower_level = 1;
                            let tower_cost = tower_type.to_cost(tower_level);
                            let tower = Tower(tower_type.to_tower_data(tower_level));
                            if let Some(texture) =
                                tower_control.textures.get(&(tower_type, tower_level))
                            {
                                commands.spawn((
                                    Sprite::from_image(texture.0.clone()),
                                    tower,
                                    Transform {
                                        translation: Vec3::new(placement.x, placement.y, 1.0),
                                        scale: Vec3::splat(2.0),
                                        ..default()
                                    },
                                ));
                                tower_control.placements[i] = 1;
                                gold.0 -= tower_cost;
                                info!("gold: {:?}", gold.0);
                                break;
                            }
                        } else {
                            info!("placement value: {:?}", tower_control.placements[i])
                        }
                    }
                }
            }
        }
    }
}
