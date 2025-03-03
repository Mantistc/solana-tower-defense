use bevy::prelude::*;

use crate::{
    enemies::{Enemy, BREAK_POINTS},
    tower_building::{DESPAWN_SHOT_RANGE, SHOT_HURT_DISTANCE, SHOT_SPEED},
};

use super::{Tower, TOWER_ATTACK_RANGE};

#[derive(Component)]
pub struct Shot {
    pub direction: Vec3,
    pub damage: u8,
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
        transform.translation += shot.direction * SHOT_SPEED * time.delta_secs();

        if transform.translation.x > DESPAWN_SHOT_RANGE {
            commands.entity(shot_entity).despawn();
        }

        let shot_position = transform.translation;
        for (enemy_entity, enemy_transform, mut enemy) in &mut enemies {
            let enemy_position = enemy_transform.translation;
            let distance = shot_position.distance_squared(enemy_position);
            if distance <= SHOT_HURT_DISTANCE {
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
