use bevy::prelude::*;

use crate::{
    enemies::{BreakPointLvl, Enemy, BREAK_POINTS},
    tower_building::{DESPAWN_SHOT_RANGE, SHOT_HURT_DISTANCE, SHOT_SPEED},
};

use super::{Gold, Tower, TOWER_ATTACK_RANGE};

#[derive(Component)]
pub struct Shot {
    pub direction: Vec3,
    pub damage: u8,
}

pub fn spawn_shots_to_attack(
    enemies: Query<(&Transform, &BreakPointLvl), (Without<Tower>, With<Enemy>)>,
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

        // the higher breakpoint lvl, the close to the victory
        // so, we needs to filter all enemies that are in the attack range
        // then, take all of the higher breakpoint lvl
        // then take the closer to the breakpoint

        let enemies_in_range: Vec<(&Transform, &BreakPointLvl)> = enemies
            .iter()
            .filter(|(t, _)| {
                let enemy_position = t.translation;
                let distance = tower_position.distance(enemy_position);
                distance < TOWER_ATTACK_RANGE && distance > 0.0
            })
            .collect();

        // get the max break lvl value
        let max_break_value = enemies_in_range
            .iter()
            .cloned()
            .map(|(_, b)| b)
            .max()
            .unwrap_or(&BreakPointLvl(0));
        // get all the enemies with this max break lvl
        let closer_enemies_to_victory: Vec<(&Transform, &BreakPointLvl)> = enemies_in_range
            .iter()
            .filter(|(_, b)| **b == *max_break_value)
            .copied()
            .collect();

        // get the closer enemy to the break point lvl
        for (enemy_transform, break_point_lvl) in &closer_enemies_to_victory {
            let index = break_point_lvl.0 as usize;
            let enemy_position = enemy_transform.translation;
            let distance_to_target = enemy_position.truncate().distance(BREAK_POINTS[index]);
            if distance_to_target < closest_distance_to_target {
                closest_distance_to_target = distance_to_target;
                target_enemy_position = Some(enemy_position);
            }
        }
        if let Some(enemy_position) = target_enemy_position {
            if tower.attack_speed.just_finished() {
                let direction = (enemy_position - tower_position).normalize();

                let shot = Shot {
                    direction,
                    damage: tower.attack_damage,
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
    mut shots: Query<(Entity, &mut Transform, &Shot)>,
    mut commands: Commands,
    mut gold: ResMut<Gold>,
    time: Res<Time>,
) {
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
                commands.entity(shot_entity).despawn();
                enemy.life = enemy.life.saturating_sub(shot.damage);
                if enemy.life <= 0 {
                    commands.entity(enemy_entity).despawn();
                    gold.0 += 10;
                }
            }
        }
    }
}
