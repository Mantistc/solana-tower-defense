use super::*;
use bevy::prelude::*;

pub fn get_enemy_list() -> Vec<(String, UVec2, u32, u32, EnemyAnimation)> {
    let columns = 4;
    let rows = 4;
    let enemy_list = vec![
        (
            "enemies/ohai.png".to_string(),
            UVec2::splat(32),
            columns,
            rows,
            ideal_animation_values(),
        ),
        (
            "enemies/micuwa.png".to_string(),
            UVec2::splat(32),
            columns,
            rows,
            ideal_animation_values(),
        ),
        (
            "enemies/soldier.png".to_string(),
            UVec2::splat(32),
            8,
            1,
            EnemyAnimation::make_all(0, 7, ideal_time_per_frame()),
        ),
        (
            "enemies/orcs.png".to_string(),
            UVec2::splat(32),
            8,
            1,
            EnemyAnimation::make_all(0, 7, ideal_time_per_frame()),
        ),
        (
            "enemies/leaf-bug.png".to_string(),
            UVec2::splat(64),
            24,
            1,
            EnemyAnimation {
                walk_up: AnimateSprite {
                    first: 8,
                    last: 15,
                    ..default()
                },
                walk_down: AnimateSprite {
                    first: 0,
                    last: 7,
                    ..default()
                },
                walk_left: AnimateSprite {
                    first: 16,
                    last: 23,
                    ..default()
                },
                need_flip: true,
                ..default()
            },
        ),
        (
            "enemies/magma-crab.png".to_string(),
            UVec2::splat(64),
            24,
            1,
            EnemyAnimation {
                walk_up: AnimateSprite {
                    first: 8,
                    last: 15,
                    ..default()
                },
                walk_down: AnimateSprite {
                    first: 0,
                    last: 7,
                    ..default()
                },
                walk_left: AnimateSprite {
                    first: 16,
                    last: 23,
                    ..default()
                },
                ..default()
            },
        ),
        (
            "enemies/fire-bug.png".to_string(),
            UVec2::new(96, 64),
            24,
            1,
            EnemyAnimation {
                walk_up: AnimateSprite {
                    first: 8,
                    last: 15,
                    ..default()
                },
                walk_down: AnimateSprite {
                    first: 0,
                    last: 7,
                    ..default()
                },
                walk_left: AnimateSprite {
                    first: 16,
                    last: 23,
                    ..default()
                },
                need_flip: true,
                ..default()
            },
        ),
    ];
    enemy_list
}
