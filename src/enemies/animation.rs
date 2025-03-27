use bevy::prelude::*;

#[derive(Clone, Debug)]
pub struct AnimateSprite {
    pub first: usize,
    pub last: usize,
    pub timer: Timer,
}

impl Default for AnimateSprite {
    fn default() -> Self {
        Self {
            first: 0,
            last: 0,
            timer: Default::default(),
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct EnemyAnimation {
    pub walk_up: AnimateSprite,
    pub walk_down: AnimateSprite,
    pub walk_left: AnimateSprite,
    pub walk_right: AnimateSprite,
    pub state: EnemyAnimationState,
}

impl Default for EnemyAnimation {
    fn default() -> Self {
        Self {
            walk_up: Default::default(),
            walk_down: Default::default(),
            walk_left: Default::default(),
            walk_right: Default::default(),
            state: EnemyAnimationState::WalkLeft,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum EnemyAnimationState {
    WalkUp,
    WalkDown,
    WalkLeft,
    WalkRight,
}

pub fn animate(
    mut enemy_animation_query: Query<(&mut Transform, &mut Sprite, &mut EnemyAnimation)>,
    time: Res<Time>,
) {
    for (mut _transform, mut enemy_sprite, mut enemy_animation) in &mut enemy_animation_query {
        let animation = match enemy_animation.state {
            EnemyAnimationState::WalkUp => &mut enemy_animation.walk_up,
            EnemyAnimationState::WalkDown => &mut enemy_animation.walk_down,
            EnemyAnimationState::WalkLeft => &mut enemy_animation.walk_right,
            EnemyAnimationState::WalkRight => &mut enemy_animation.walk_left,
        };

        animation.timer.tick(time.delta());

        if animation.timer.just_finished() {
            if let Some(atlas) = &mut enemy_sprite.texture_atlas {
                atlas.index = if atlas.index < animation.first || atlas.index >= animation.last {
                    animation.first
                } else {
                    atlas.index + 1
                };
            };
        }
    }
}
