use bevy::prelude::*;

use super::{
    animate, load_enemy_sprites, EnemyAnimation, GameState, WaveControl, MAX_ENEMIES_PER_WAVE,
    SCALE, SPAWN_X_LOCATION, SPAWN_Y_LOCATION,
};

// define plugin
pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(Startup, load_enemy_sprites)
            .add_systems(
                Update,
                (spawn, animate, move_enemies, despawn_enemies, wave_control),
            );
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
            speed: 75.0,
        }
    }
}

// define systems
fn spawn(mut commands: Commands, time: Res<Time>, mut wave_control: ResMut<WaveControl>) {
    wave_control.time_between_spawns.tick(time.delta());

    if wave_control.time_between_spawns.just_finished()
        && wave_control.spawned_count_in_wave < MAX_ENEMIES_PER_WAVE
    {
        wave_control.time_between_waves.reset();
        wave_control.time_between_waves.pause();
        let wave_image = &wave_control.textures[wave_control.wave_count as usize];
        let enemy_animation = &wave_control.animations[wave_control.wave_count as usize];
        commands.spawn((
            Sprite::from_atlas_image(
                wave_image.0.clone(),
                TextureAtlas {
                    layout: wave_image.1.clone(),
                    index: enemy_animation.walk.first,
                },
            ),
            Transform {
                translation: Vec3::new(SPAWN_X_LOCATION, SPAWN_Y_LOCATION, 1.0),
                scale: Vec3::new(-SCALE, SCALE, 0.0),
                ..default()
            },
            Enemy::default(),
            enemy_animation.clone(),
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

pub fn move_enemies(mut enemies: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut enemy_transform, enemy) in &mut enemies {
        let translation = enemy_transform.translation;
        let speed = enemy.speed * time.delta_secs();

        // 1. if x > BREAK_POINTS[0], move -x
        if translation.x > BREAK_POINTS[0] {
            enemy_transform.translation.x -= speed;
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
    for (enemy_transform, entity) in &mut enemies {
        let translation = enemy_transform.translation;
        if translation.y <= BREAK_POINTS[4] {
            commands.entity(entity).despawn();
        }
    }
}

pub fn wave_control(
    time: Res<Time>,
    mut wave_control: ResMut<WaveControl>,
    enemies: Query<Entity, With<Enemy>>,
) {
    let current_wave_enemies: usize = enemies.iter().len();
    info!(
        "e: {}, wave: {}",
        current_wave_enemies, wave_control.wave_count
    );
    if wave_control.spawned_count_in_wave == MAX_ENEMIES_PER_WAVE && current_wave_enemies == 0 {
        if wave_control.time_between_waves.elapsed_secs() == 0.0 {
            info!("unpaused");
            wave_control.time_between_waves.unpause();
            wave_control.time_between_waves.reset();
        }

        wave_control.time_between_waves.tick(time.delta());
        info!(
            "elapsed time: {:.2} / 30.0",
            wave_control.time_between_waves.elapsed_secs()
        );
        if wave_control.time_between_waves.just_finished() {
            info!("restarted");
            wave_control.spawned_count_in_wave = 0;
            wave_control.wave_count += 1;
        }
    }
}
