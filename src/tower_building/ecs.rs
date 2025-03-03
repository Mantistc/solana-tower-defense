use bevy::prelude::*;

use crate::enemies::Enemy;

use super::{SPAWN_X_LOCATION, SPAWN_Y_LOCATION, TOWER_ATTACK_RANGE};

pub struct TowersPlugin;

impl Plugin for TowersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn)
            .add_systems(Update, (spawn_shots_to_attack, shot_enemies));
    }
}

#[derive(Component, Debug)]
pub struct Tower {
    pub attack_damage: u8,
    pub attack_speed: Timer,
}

#[derive(Component)]
pub struct Shot {
    pub speed: f32,
    pub direction: Vec3,
}

impl Default for Tower {
    fn default() -> Self {
        Self {
            attack_speed: Timer::from_seconds(0.25, TimerMode::Repeating),
            attack_damage: 5,
        }
    }
}

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("player/player_sprite_sheet.png"); // simulate this is a tower
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 6, 6, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        ),
        Tower::default(),
        Transform {
            translation: Vec3::new(SPAWN_X_LOCATION, SPAWN_Y_LOCATION, 1.0),
            scale: Vec3::splat(2.0),
            ..default()
        },
    ));
}

pub fn spawn_shots_to_attack(
    enemies: Query<&Transform, (Without<Tower>, With<Enemy>)>,
    mut towers: Query<(&Transform, &mut Tower)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let there_is_enemies = enemies.iter().len();
    if there_is_enemies > 0 {
        for (tower_transform, mut tower) in &mut towers {
            let tower_position = tower_transform.translation;
            tower.attack_speed.tick(time.delta());

            let mut can_attack = false;
            let mut closest_enemy_position = Vec3::ZERO;
            let mut closest_distance = f32::MAX;

            for enemy_transform in &enemies {
                let enemy_position = enemy_transform.translation;
                let distance = tower_position.distance(enemy_position);

                if distance < TOWER_ATTACK_RANGE && distance > 0.0 {
                    if distance < closest_distance {
                        closest_distance = distance;
                        closest_enemy_position = enemy_position;
                        can_attack = true;
                    }
                }
            }

            if can_attack && tower.attack_speed.just_finished() {
                info!("spawned_shot at distance: {:?}", closest_distance);

                let direction = (closest_enemy_position - tower_position).normalize();

                let shot = Shot {
                    speed: 700.0,
                    direction,
                };

                commands.spawn((
                    Sprite {
                        color: Color::srgb(1.0, 0.0, 0.0),
                        custom_size: Some(Vec2::new(10.0, 10.0)),
                        ..default()
                    },
                    shot,
                    Transform {
                        translation: tower_position,
                        ..default()
                    },
                ));
            }
        }
    }
}

pub fn shot_enemies(
    mut enemies: Query<(Entity, &Transform, &mut Enemy), Without<Shot>>,
    mut shots: Query<(Entity, &mut Transform, &Shot)>,
    tower: Query<&Tower>,
    mut commands: Commands,
    time: Res<Time>,
) {
    if let Ok(tower) = tower.get_single() {
        for (shot_entity, mut transform, shot) in &mut shots {
            transform.translation += shot.direction * shot.speed * time.delta_secs();

            if transform.translation.x > 800.0 {
                commands.entity(shot_entity).despawn();
            }

            let shot_position = transform.translation;
            for (enemy_entity, enemy_transform, mut enemy) in &mut enemies {
                let enemy_position = enemy_transform.translation;
                let distance = shot_position.distance_squared(enemy_position);
                if distance <= 700.0 {
                    info!("shotted: {:?}", true);
                    commands.entity(shot_entity).despawn();
                    enemy.life -= tower.attack_damage;
                    if enemy.life == 0 {
                        commands.entity(enemy_entity).despawn();
                    }
                }
            }
        }
    }
}
