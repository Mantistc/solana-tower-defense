use bevy::prelude::*;

use crate::{
    animations::{animate_orcs, OrcsAnimation, OrcsAnimationState},
    player::Player,
};

use super::Enemy;

// define plugin
pub struct OrcsPlugin;

impl Plugin for OrcsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_orcs)
            .add_systems(Update, (animate_orcs, follow_player, attack));
    }
}

// define components

#[derive(Component)]
pub struct Orcs {
    pub life: u8,
    pub attack_damage: u8,
    pub speed: f32,
    pub attack_cooldown: Timer,
}

impl Default for Orcs {
    fn default() -> Self {
        Self {
            life: 100,
            attack_damage: 10,
            speed: 65.0,
            attack_cooldown: Timer::from_seconds(1.5, TimerMode::Repeating),
        }
    }
}

// define systems
const SPAWN_AMOUNT: u8 = 10;
pub fn spawn_orcs(
    mut commands: Commands,
    ref asset_server: Res<AssetServer>,
    ref mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let orcs_animation = OrcsAnimation::set(asset_server, texture_atlas_layouts);

    for i in 0..SPAWN_AMOUNT {
        commands.spawn((
            Sprite::from_atlas_image(
                orcs_animation
                    .idle
                    .sprite_texture_atlas
                    .as_deref()
                    .expect("Initializing...")
                    .1
                    .clone(),
                orcs_animation
                    .idle
                    .sprite_texture_atlas
                    .as_deref()
                    .expect("Initializing...")
                    .0
                    .clone(),
            ),
            Transform {
                translation: Vec3::new(150.0 * i as f32, -125.0, 1.0),
                scale: Vec3::splat(2.0),
                ..default()
            },
            Orcs::default(),
            Enemy,
            orcs_animation.clone(),
        ));
    }
}

pub fn attack() {}

const MAX_AGRO_DISTANCE: f32 = 25.0;

pub fn follow_player(
    player: Query<&Transform, With<Player>>,
    mut orcs: Query<(&mut Transform, &Orcs, &mut OrcsAnimation), Without<Player>>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player.get_single() {
        let player_position = player_transform.translation.truncate();

        for (mut orc_transform, orcs, mut orcs_animation) in &mut orcs {
            let orc_position = orc_transform.translation.truncate();
            let direction = (player_position - orc_position).normalize_or_zero();
            let distance = orc_position.distance(player_position);
            if distance < MAX_AGRO_DISTANCE {
                orcs_animation.state = OrcsAnimationState::Walk;
                let orcs_speed = orcs.speed * time.delta_secs();
                orc_transform.translation.x += direction.x * orcs_speed;
                orc_transform.translation.y += direction.y * orcs_speed;
            } else {
                orcs_animation.state = OrcsAnimationState::Idle;
            }
        }
    }
}
