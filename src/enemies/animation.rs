use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct AnimateSprite {
    pub first: usize,
    pub last: usize,
    pub timer: Timer,
}

#[derive(Component, Clone, Resource)]
pub struct EnemyAnimation {
    pub walk: AnimateSprite,
    pub idle: AnimateSprite,
    pub attack: AnimateSprite,
    pub death: AnimateSprite,
    pub state: EnemyAnimationState,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum EnemyAnimationState {
    Walk,
    Idle,
    Attack,
    Death,
}

impl Default for EnemyAnimation {
    fn default() -> Self {
        Self {
            idle: AnimateSprite {
                first: 0,
                last: 5,
                timer: Timer::from_seconds(0.25, TimerMode::Repeating),
            },
            walk: AnimateSprite {
                first: 8,
                last: 15,
                timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            },
            attack: AnimateSprite {
                first: 16,
                last: 21,
                timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            },
            death: AnimateSprite {
                first: 40,
                last: 43,
                timer: Timer::from_seconds(0.25, TimerMode::Repeating),
            },
            state: EnemyAnimationState::Idle,
        }
    }
}

pub fn animate(
    mut enemy_animation_query: Query<(&mut Transform, &mut Sprite, &mut EnemyAnimation)>,
    time: Res<Time>,
) {
    for (mut _transform, mut orc_sprite, mut orc_animation) in &mut enemy_animation_query {
        let animation = match orc_animation.state {
            EnemyAnimationState::Walk => &mut orc_animation.walk,
            EnemyAnimationState::Idle => &mut orc_animation.idle,
            EnemyAnimationState::Attack => &mut orc_animation.attack,
            EnemyAnimationState::Death => &mut orc_animation.death,
        };
        animation.timer.tick(time.delta());

        if animation.timer.just_finished() {
            if let Some(atlas) = &mut orc_sprite.texture_atlas {
                atlas.index = if atlas.index < animation.first || atlas.index >= animation.last {
                    animation.first
                } else {
                    atlas.index + 1
                };
            };
        }
    }
}
