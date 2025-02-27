use bevy::prelude::*;

use super::AnimateSprite;

#[derive(Component)]
pub struct PlayerAnimations {
    pub running: AnimateSprite,
    pub idle: AnimateSprite,
}

impl Default for PlayerAnimations {
    fn default() -> Self {
        Self {
            running: AnimateSprite {
                first: 6,
                last: 11,
                timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            },
            idle: AnimateSprite {
                first: 0,
                last: 3,
                timer: Timer::from_seconds(0.25, TimerMode::Repeating),
            },
        }
    }
}

pub fn animate_player(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Sprite, &mut PlayerAnimations)>,
) {
    if let Ok((mut sprite, mut animations)) = query.get_single_mut() {
        let moving = input.pressed(KeyCode::KeyW)
            || input.pressed(KeyCode::KeyA)
            || input.pressed(KeyCode::KeyD)
            || input.pressed(KeyCode::KeyS);

        let animation = if moving {
            &mut animations.running
        } else {
            &mut animations.idle
        };

        animation.timer.tick(time.delta());

        if animation.timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index < animation.first || atlas.index >= animation.last {
                    animation.first
                } else {
                    atlas.index + 1
                };
            };
        }
    }
}
