use bevy::prelude::*;

use super::{Gold, SelectedTowerType, TowerControl, TowerType, TOWER_POSITION_PLACEMENT};

#[derive(Debug, Clone)]
pub struct TowerInfo {
    pub attack_damage: u16,
    pub attack_speed: Timer,
    pub level: u8,
    pub tower_type: TowerType,
}

#[derive(Component, Debug, Deref, DerefMut)]
pub struct Tower(pub TowerInfo);

pub fn buy_tower(
    windows: Query<&Window>,
    buttons: Res<ButtonInput<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut commands: Commands,
    mut tower_control: ResMut<TowerControl>,
    mut gold: ResMut<Gold>,
    selected_tower_type: Res<SelectedTowerType>,
    mut placement_zones: Query<(&Transform, &mut Sprite), With<TowerPlacementZone>>,
) {
    let window = windows.single();
    let range = 32.0;

    if let Some(cursor_position) = window.cursor_position() {
        if let Ok((camera, camera_transform)) = camera_query.get_single() {
            if let Ok(world_position) = camera.viewport_to_world(camera_transform, cursor_position)
            {
                let cursor_world_pos = world_position.origin.truncate(); // Vec2
                for (i, placement) in TOWER_POSITION_PLACEMENT.iter().enumerate() {
                    let in_range = cursor_world_pos.x >= placement.x - range
                        && cursor_world_pos.x <= placement.x + range
                        && cursor_world_pos.y >= placement.y - range
                        && cursor_world_pos.y <= placement.y + range;

                    let tower_level = 1;
                    let tower_cost = selected_tower_type.to_cost(tower_level);

                    if let Some(&zone_entity) = tower_control.zones.get(i) {
                        if let Ok((_, mut sprite)) = placement_zones.get_mut(zone_entity) {
                            sprite.color = if in_range && gold.0 >= tower_cost {
                                info!("gold: {:?}", gold.0);
                                Color::srgba(0.0, 1.0, 0.0, 0.5)
                            } else {
                                info!("gold: {:?}", gold.0);
                                Color::srgba(0.0, 1.0, 0.0, 0.0)
                            };
                        }
                    }

                    if in_range
                        && tower_control.placements[i] == 0
                        && buttons.just_pressed(MouseButton::Left)
                        && gold.0 >= tower_cost
                    {
                        let tower = Tower(selected_tower_type.to_tower_data(tower_level));

                        if gold.0 < tower_cost {
                            return;
                        }

                        if let Some(texture) = tower_control
                            .textures
                            .get(&(selected_tower_type.0.clone(), tower_level))
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
                    }
                }
            }
        }
    }
}

pub fn upgrade_tower() {}

pub fn select_tower_type(
    mut selected_tower_type: ResMut<SelectedTowerType>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyZ) {
        selected_tower_type.0 = TowerType::Zigurat;
    }
    if input.just_pressed(KeyCode::KeyE) {
        selected_tower_type.0 = TowerType::Electric;
    }
    if input.just_pressed(KeyCode::KeyL) {
        selected_tower_type.0 = TowerType::Lich;
    }
}

#[derive(Component)]
pub struct TowerPlacementZone;

pub fn setup_tower_zones(mut commands: Commands, mut tower_control: ResMut<TowerControl>) {
    for placement in TOWER_POSITION_PLACEMENT.iter() {
        let entity = commands
            .spawn((
                Sprite {
                    color: Color::srgba(0.0, 1.0, 0.0, 0.0),
                    custom_size: Some(Vec2::splat(64.0)),
                    ..default()
                },
                Transform {
                    translation: Vec3::new(placement.x, placement.y, 0.5),
                    ..default()
                },
                TowerPlacementZone,
            ))
            .id();
        tower_control.zones.push(entity);
    }
}
