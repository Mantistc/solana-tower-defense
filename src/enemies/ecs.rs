use bevy::prelude::*;

use super::{
    animate, load_enemy_sprites, EnemyAnimation, EnemyAnimationState, GameState, WaveControl,
    MAX_ENEMIES_PER_WAVE, SPAWN_Y_LOCATION,
};

// define plugin
pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(Startup, load_enemy_sprites)
            .add_systems(Update, (spawn, animate, move_enemies, despawn_enemies));
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
fn spawn(mut commands: Commands, time: Res<Time>, mut wave_control: ResMut<WaveControl>) {
    wave_control.time_between_spawns.tick(time.delta());

    if wave_control.time_between_spawns.just_finished()
        && wave_control.spawned_count_in_wave < MAX_ENEMIES_PER_WAVE
    {
        let wave_image = &wave_control.textures[wave_control.wave_count as usize];

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
            EnemyAnimation::default(),
        ));

        wave_control.spawned_count_in_wave += 1;
    }
}

//
const BREAK_POINTS: [f32; 5] = [
    250.0,  // x check
    -125.0, // y check
    -230.0, // x check
    -455.0, // x check
    -295.0, // y final part
]; //

pub fn move_enemies(
    mut enemies: Query<(&mut Transform, &Enemy, &mut EnemyAnimation)>,
    time: Res<Time>,
) {
    for (mut enemy_transform, enemy, mut enemy_animation) in &mut enemies {
        let translation = enemy_transform.translation;
        let speed = enemy.speed * time.delta_secs();

        // 1. if x > BREAK_POINTS[0], move -x
        if translation.x > BREAK_POINTS[0] {
            enemy_transform.translation.x -= speed;
            info!("1");
        }
        // 2. if x <= BREAK_POINTS[0], move en -y
        else if translation.x <= BREAK_POINTS[0]
            && translation.x > BREAK_POINTS[2]
            && translation.y > BREAK_POINTS[1]
        {
            enemy_transform.translation.y -= speed;
        }
        // 3. if y <= BREAK_POINTS[1] && x >= BREAK_POINTS[2], move -x
        else if translation.y <= BREAK_POINTS[1] && translation.x >= BREAK_POINTS[2] {
            enemy_transform.translation.x -= speed;
        }
        // 4. if y < SPAWN_Y_LOCATION && x <= BREAK_POINTS[2], move +y
        else if translation.y < SPAWN_Y_LOCATION
            && translation.x <= BREAK_POINTS[2]
            && translation.x > BREAK_POINTS[3]
        {
            enemy_transform.translation.y += speed;
        }
        // 5. if y >= SPAWN_Y_LOCATION && x >= BREAK_POINTS[3], move -x
        else if translation.y >= SPAWN_Y_LOCATION && translation.x >= BREAK_POINTS[3] {
            enemy_transform.translation.x -= speed;
        }
        // 6. if y > BREAK_POINTS[4] && x <= BREAK_POINTS[3], move -y
        else if translation.y > BREAK_POINTS[4] && translation.x <= BREAK_POINTS[3] {
            enemy_transform.translation.y -= speed;
        }
    }
}

pub fn despawn_enemies(
    mut commands: Commands,
    mut enemies: Query<(&Transform, Entity), With<Enemy>>,
) {
    for (enemy_transform, mut entity) in &mut enemies {
        let translation = enemy_transform.translation;
        if translation.y <= BREAK_POINTS[4] {
            commands.entity(entity).despawn();
        }
    }
}
