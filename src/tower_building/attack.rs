use bevy::prelude::*;

use crate::{
    enemies::{BreakPointLvl, Enemy, WaveControl, BREAK_POINTS},
    tower_building::{DESPAWN_SHOT_RANGE, SHOT_HURT_DISTANCE, SHOT_SPEED},
};

use super::{Gold, Tower, TOWER_ATTACK_RANGE};

#[derive(Component)]
pub struct Shot {
    pub direction: Vec3,
    pub damage: u16,
    pub target: Option<Entity>,
}

pub fn spawn_shots_to_attack(
    enemies: Query<(&Transform, &BreakPointLvl, Entity), (Without<Tower>, With<Enemy>)>,
    mut towers: Query<(&Transform, &mut Tower)>,
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    for (tower_transform, mut tower) in &mut towers {
        let tower_position = tower_transform.translation;
        tower.attack_speed.tick(time.delta());

        let mut target_enemy_position = None;
        let mut closest_distance_to_target = f32::MAX;

        // the higher breakpoint lvl, the close is the enemy to the victory
        // so, we need to filter all enemies that are in the attack range
        // then, take all of the higher breakpoint lvl
        // then take the closest to the breakpoint

        let enemies_in_range: Vec<(&Transform, &BreakPointLvl, Entity)> = enemies
            .iter()
            .filter(|(t, _, _)| {
                let enemy_position = t.translation;
                let distance = tower_position.distance(enemy_position);
                distance < TOWER_ATTACK_RANGE && distance > 0.0
            })
            .collect();

        // get the max break lvl value
        let max_break_value = enemies_in_range
            .iter()
            .cloned()
            .map(|(_, b, _)| b)
            .max()
            .unwrap_or(&BreakPointLvl(0));

        // get all the enemies with this max break lvl
        let closer_enemies_to_victory: Vec<(&Transform, &BreakPointLvl, Entity)> = enemies_in_range
            .iter()
            .filter(|(_, b, _)| **b == *max_break_value)
            .copied()
            .collect();

        // get the closest enemy to the break point lvl
        let mut closest_enemy = None;
        for (enemy_transform, break_point_lvl, enemy_entity) in &closer_enemies_to_victory {
            let index = break_point_lvl.0 as usize;
            let enemy_position = enemy_transform.translation;
            let distance_to_target = enemy_position.truncate().distance(BREAK_POINTS[index]);

            if distance_to_target < closest_distance_to_target {
                closest_distance_to_target = distance_to_target;
                target_enemy_position = Some(enemy_position);
                closest_enemy = Some(enemy_entity)
            }
        }
        if let Some(enemy_position) = target_enemy_position {
            if tower.attack_speed.just_finished() {
                let direction = (enemy_position - tower_position).normalize();

                let shot = Shot {
                    direction,
                    damage: tower.attack_damage,
                    target: Some(*closest_enemy.unwrap()),
                };
                let texture = asset_server.load("towers/lich_01_shot.png");
                commands.spawn((
                    Sprite {
                        custom_size: Some(Vec2::new(64.0, 64.0)),
                        ..Sprite::from_image(texture)
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
    mut shots: Query<(Entity, &mut Transform, &mut Shot)>,
    mut commands: Commands,
    mut gold: ResMut<Gold>,
    time: Res<Time>,
    wave_control: Res<WaveControl>,
) {
    for (shot_entity, mut transform, shot) in &mut shots {
        let mut hit_enemy = None;
        let mut closest_distance = f32::MAX;
        let next_position = shot.direction * SHOT_SPEED * time.delta_secs();
        if let Some(target_entity) = shot.target {
            if let Ok((_, enemy_transform, _)) = enemies.get(target_entity) {
                let direction = (enemy_transform.translation - transform.translation).normalize();
                transform.translation += direction * SHOT_SPEED * time.delta_secs();
            } else {
                transform.translation += next_position;
            }
        } else {
            transform.translation += next_position;
        }

        for (enemy_entity, enemy_transform, enemy) in &mut enemies {
            let enemy_position = enemy_transform.translation;
            let distance = transform.translation.distance_squared(enemy_position);

            if distance <= SHOT_HURT_DISTANCE && distance < closest_distance {
                hit_enemy = Some((enemy_entity, enemy));
                closest_distance = distance;
            }
        }

        if let Some((enemy_entity, mut enemy)) = hit_enemy {
            commands.entity(shot_entity).despawn();
            enemy.life = enemy.life.saturating_sub(shot.damage);
            if enemy.life <= 0 {
                commands.entity(enemy_entity).despawn();

                let wave_factor = wave_control.wave_count as f32 + 1.0;
                let gold_reward = ((enemy.life as f32 / 2.5) + (wave_factor * 2.0)).round() as u16;

                gold.0 += gold_reward;
                info!("Enemy killed! Gained {} gold.", gold_reward);
            }
        } else if transform.translation.x > DESPAWN_SHOT_RANGE {
            commands.entity(shot_entity).despawn();
        }
    }
}
