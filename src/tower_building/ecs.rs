use bevy::prelude::*;

use crate::enemies::{Enemy, BREAK_POINTS};

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
    pub damage: u8,
}

impl Default for Tower {
    fn default() -> Self {
        Self {
            attack_speed: Timer::from_seconds(0.25, TimerMode::Repeating),
            attack_damage: 5,
        }
    }
}

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
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

pub fn spawn_shots_to_attack(
    enemies: Query<&Transform, (Without<Tower>, With<Enemy>)>,
    mut towers: Query<(&Transform, &mut Tower)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (tower_transform, mut tower) in &mut towers {
        let tower_position = tower_transform.translation;
        tower.attack_speed.tick(time.delta());

        let mut target_enemy_position = None;
        let mut closest_distance_to_target = f32::MAX;

        for enemy_transform in &enemies {
            let enemy_position = enemy_transform.translation;
            let distance = tower_position.distance(enemy_position);
            let distance_to_target = enemy_position
                .truncate()
                .distance(Vec2::new(BREAK_POINTS[3], BREAK_POINTS[4]));

            if distance < TOWER_ATTACK_RANGE && distance > 0.0 {
                if distance_to_target < closest_distance_to_target {
                    closest_distance_to_target = distance_to_target;
                    target_enemy_position = Some(enemy_position);
                }
            }
        }
        if let Some(enemy_position) = target_enemy_position {
            if tower.attack_speed.just_finished() {
                info!("spawned_shot at enemy_position: {:?}", enemy_position);

                let direction = (enemy_position - tower_position).normalize();

                let shot = Shot {
                    speed: 700.0,
                    direction,
                    damage: tower.attack_damage,
                };

                commands.spawn((
                    Sprite {
                        color: Color::srgb(0.0, 0.0, 0.0),
                        custom_size: Some(Vec2::new(10.0, 10.0)),
                        ..default()
                    },
                    shot,
                    Transform {
                        translation: Vec3::new(tower_position.x, tower_position.y, 1.5),
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
    mut commands: Commands,
    time: Res<Time>,
) {
    let shots_len = shots.iter().len();
    info!("shots: {:?}", shots_len);
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
                enemy.life = enemy.life.saturating_sub(shot.damage);
                if enemy.life <= 0 {
                    commands.entity(enemy_entity).despawn();
                }
            }
        }
    }
}
