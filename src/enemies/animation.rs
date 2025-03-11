use bevy::prelude::*;

#[derive(Clone, Debug)]
pub struct AnimateSprite {
    pub first: usize,
    pub last: usize,
    pub timer: Timer,
}

#[derive(Component, Clone, Debug)]
pub struct EnemyAnimation {
    pub walk: AnimateSprite,
    pub death: AnimateSprite,
    pub state: EnemyAnimationState,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum EnemyAnimationState {
    Walk,
    Death,
}

pub fn animate(
    mut enemy_animation_query: Query<(&mut Transform, &mut Sprite, &mut EnemyAnimation)>,
    time: Res<Time>,
) {
    for (mut _transform, mut enemy_sprite, mut enemy_animation) in &mut enemy_animation_query {
        let animation = match enemy_animation.state {
            EnemyAnimationState::Walk => &mut enemy_animation.walk,
            EnemyAnimationState::Death => &mut enemy_animation.death,
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
