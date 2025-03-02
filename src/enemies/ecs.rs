use bevy::prelude::*;

use super::{animate, load_enemy_sprites, EnemyAnimation, WaveControl, MAX_ENEMIES_PER_WAVE};

// define plugin
pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (load_enemy_sprites, spawn))
            .add_systems(Update, (animate, move_enemies));
    }
}

// define components

#[derive(Component)]
pub struct Enemy {
    pub life: u8,
    pub speed: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            life: 25,
            speed: 125.0,
        }
    }
}

// define systems
pub fn spawn(
    mut commands: Commands,
    wave_control: Res<WaveControl>,
) {
    let wave_image = &wave_control.textures[wave_control.wave_count as usize];

    for _i in 0..MAX_ENEMIES_PER_WAVE {
        commands.spawn((
            Sprite::from_atlas_image(
                wave_image.0.clone(),
                TextureAtlas {
                    layout: wave_image.1.clone(),
                    index: 0,
                },
            ),
            Transform {
                translation: Vec3::new(610.0, 150.0, 1.0),
                scale: Vec3::new(-2.0, 2.0, 0.0),
                ..default()
            },
            Enemy::default(),
            EnemyAnimation::default()
        ));
    }
}

pub fn move_enemies(mut enemies: Query<(&mut Transform, &Enemy, &mut EnemyAnimation)>, time: Res<Time>) {

}
