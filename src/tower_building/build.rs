use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

use crate::tilemap::TILE_SIZE;

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
                                Color::srgba(0.0, 1.0, 0.0, 0.5)
                            } else if in_range && gold.0 < tower_cost {
                                Color::srgba(1.0, 0.0, 0.0, 0.5)
                            } else {
                                Color::srgba(0.0, 0.0, 0.0, 0.0)
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
                            info!("insufficient gold: {:?}", gold.0);
                            return;
                        }

                        if let Some(texture) = tower_control
                            .textures
                            .get(&(selected_tower_type.0.clone(), tower_level))
                        {
                            commands.spawn((
                                Sprite::from_image(texture.clone()),
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

pub fn upgrade_tower(
    windows: Query<&Window>,
    buttons: Res<ButtonInput<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    tower_control: ResMut<TowerControl>,
    mut gold: ResMut<Gold>,
    mut towers: Query<(&Transform, &mut Sprite, &mut Tower)>,
) {
    let window = windows.single();
    if let Some(cursor_position) = window.cursor_position() {
        if let Ok((camera, camera_transform)) = camera_query.get_single() {
            if let Ok(world_position) = camera.viewport_to_world(camera_transform, cursor_position)
            {
                let cursor_world_pos = world_position.origin.truncate();

                for (transform, mut sprite, mut tower) in &mut towers {
                    if is_cursor_over_entity(
                        transform.translation.truncate(),
                        &sprite,
                        cursor_world_pos,
                    ) {
                        if tower.level == 3 {
                            return;
                        }
                        let next_lvl = tower.level + 1;
                        let tower_type = tower.tower_type.clone();
                        let tower_cost = tower_type.to_cost(next_lvl);
                        let tower_info = Tower(tower_type.to_tower_data(next_lvl));
                        if buttons.just_pressed(MouseButton::Left) && gold.0 >= tower_cost {
                            if let Some(texture) =
                                tower_control.textures.get(&(tower_type, next_lvl))
                            {
                                sprite.image = texture.clone();
                                info!(
                                    "gold before up: {:?}, tower damage before up {:?}, attack speed: {:?}",
                                    gold.0, tower.attack_damage, tower.attack_speed
                                );
                                gold.0 -= tower_cost;
                                *tower = tower_info;
                                info!(
                                    "gold after up: {:?}, tower damage after up {:?}, attack speed: {:?}",
                                    gold.0, tower.attack_damage, tower.attack_speed
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

fn is_cursor_over_entity(entity_pos: Vec2, sprite: &Sprite, cursor_pos: Vec2) -> bool {
    let size = sprite.custom_size.unwrap_or(Vec2::new(64.0, 64.0));
    let half_size = size / 2.0;
    let min = entity_pos - half_size;
    let max = entity_pos + half_size;

    cursor_pos.x >= min.x && cursor_pos.x <= max.x && cursor_pos.y >= min.y && cursor_pos.y <= max.y
}

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

pub fn setup_tower_zones(
    mut commands: Commands,
    mut tower_control: ResMut<TowerControl>,
    existing_zones: Query<&Transform, With<TowerPlacementZone>>,
) {
    for placement in TOWER_POSITION_PLACEMENT.iter() {
        let placement_pos = Vec3::new(placement.x, placement.y, 0.5);

        let already_exists = existing_zones
            .iter()
            .any(|t| t.translation == placement_pos);
        if already_exists {
            continue;
        }

        let entity = commands
            .spawn((
                Sprite {
                    color: Color::srgba(0.0, 0.0, 0.0, 0.0),
                    custom_size: Some(Vec2::splat(64.0)),
                    ..default()
                },
                TowerPlacementZone,
                Transform {
                    translation: placement_pos,
                    ..default()
                },
            ))
            .id();

        tower_control.zones.push(entity);
    }
}
pub fn reset_hover_color_in_attacking(
    mut placement_zones: Query<&mut Sprite, With<TowerPlacementZone>>,
) {
    for mut placements in &mut placement_zones {
        placements.color = Color::srgba(0.0, 0.0, 0.0, 0.0);
    }
}

// TODO: set the attack points based on the specific layer of the tiled map provided
pub fn _set_attack_points(
    trigger: Trigger<TiledLayerCreated>,
    map_asset: Res<Assets<TiledMap>>,
    mut commands: Commands,
) {
    let layer = trigger.event().layer(&map_asset);
    if layer.name == "attack_points" {
        if let Some(tile_layer) = layer.as_tile_layer() {
            let width = tile_layer.width().unwrap_or(0);
            let height = tile_layer.height().unwrap_or(0);
            info!("attack_points dimensions: w={}, h={}", width, height);

            for y in 0..height {
                for x in 0..width {
                    if let Some(tile) = tile_layer.get_tile(x as i32, y as i32) {
                        let tile_gid = tile.id();
                        let world_x = ((x as f32 + 1f32) - 20f32) * TILE_SIZE * 2.0;
                        let world_y = (y as f32 - 10f32) * TILE_SIZE * 2.0;

                        info!("world_x {}: world_y {}", world_x, world_y);
                        commands.spawn((
                            Sprite {
                                color: Color::srgba(0.0, 0.0, 0.0, 0.0),
                                custom_size: Some(Vec2::splat(TILE_SIZE * 2.0)),
                                ..default()
                            },
                            TowerPlacementZone,
                            Transform {
                                translation: Vec3::new(world_x, -world_y, 0.5),
                                ..default()
                            },
                        ));

                        info!("tile {} at position ({}, {})", tile_gid, x, y);
                    }
                }
            }
        }
    }
}
