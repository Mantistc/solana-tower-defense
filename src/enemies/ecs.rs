use bevy::prelude::*;

use crate::tower_building::{GameState, Lifes};

use super::{
    animate, control_first_wave, load_enemy_sprites, WaveControl, MAX_ENEMIES_PER_WAVE, SCALE,
    SPAWN_X_LOCATION, SPAWN_Y_LOCATION,
};

// define plugin
pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_enemy_sprites)
            .add_systems(
                Update,
                control_first_wave.run_if(in_state(GameState::Building)),
            )
            .add_systems(
                Update,
                wave_control
                    .run_if(in_state(GameState::Building).or(in_state(GameState::Attacking))),
            )
            .add_systems(
                Update,
                (spawn, animate, move_enemies, despawn_enemies)
                    .run_if(in_state(GameState::Attacking)),
            );
    }
}

// define components

#[derive(Component)]
pub struct Enemy {
    pub life: u8,
    pub speed: f32,
}

#[derive(Debug, Component, Deref, DerefMut, PartialEq, Eq, PartialOrd, Ord)]
pub struct BreakPointLvl(pub u8);

// define systems
fn spawn(mut commands: Commands, time: Res<Time>, mut wave_control: ResMut<WaveControl>) {
    if wave_control.wave_count == wave_control.textures.len() as u8 {
        return;
    }
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
            Enemy {
                life: 50 * (wave_control.wave_count + 1),
                speed: 75.0,
            },
            enemy_animation.clone(),
            BreakPointLvl(0),
        ));

        wave_control.spawned_count_in_wave += 1;
    }
}

pub const BREAK_POINTS: [Vec2; 6] = [
    Vec2::new(250.0, SPAWN_Y_LOCATION),
    Vec2::new(250.0, -125.0),
    Vec2::new(-230.0, -125.0),
    Vec2::new(-230.0, SPAWN_Y_LOCATION),
    Vec2::new(-455.0, SPAWN_Y_LOCATION),
    Vec2::new(-455.0, -295.0),
];

pub fn move_enemies(
    mut enemies: Query<(&mut Transform, &Enemy, &mut BreakPointLvl)>,
    time: Res<Time>,
) {
    for (mut enemy_transform, enemy, mut breal_point_lvl) in &mut enemies {
        let translation = enemy_transform.translation;
        let speed = enemy.speed * time.delta_secs();

        // 1. if x > BREAK_POINTS[0], move -x
        if translation.x > BREAK_POINTS[0].x {
            enemy_transform.translation.x -= speed;
        }
        // 2. if x <= BREAK_POINTS[0], move en -y
        else if translation.x <= BREAK_POINTS[0].x
            && translation.x > BREAK_POINTS[2].x
            && translation.y > BREAK_POINTS[1].y
        {
            enemy_transform.translation.y -= speed;
            *breal_point_lvl = BreakPointLvl(1);
        }
        // 3. if y <= BREAK_POINTS[1] && x >= BREAK_POINTS[2], move -x
        else if translation.y <= BREAK_POINTS[1].y && translation.x >= BREAK_POINTS[2].x {
            enemy_transform.translation.x -= speed;
            *breal_point_lvl = BreakPointLvl(2);
        }
        // 4. if y < SPAWN_Y_LOCATION && x <= BREAK_POINTS[2], move +y
        else if translation.y < SPAWN_Y_LOCATION
            && translation.x <= BREAK_POINTS[2].x
            && translation.x > BREAK_POINTS[4].x
        {
            enemy_transform.translation.y += speed;
            *breal_point_lvl = BreakPointLvl(3);
        }
        // 5. if y >= SPAWN_Y_LOCATION && x >= BREAK_POINTS[3], move -x
        else if translation.y >= SPAWN_Y_LOCATION && translation.x >= BREAK_POINTS[4].x {
            enemy_transform.translation.x -= speed;
            *breal_point_lvl = BreakPointLvl(4);
        }
        // 6. if y > BREAK_POINTS[4] && x <= BREAK_POINTS[3], move -y
        else if translation.y > BREAK_POINTS[5].y && translation.x <= BREAK_POINTS[4].x {
            enemy_transform.translation.y -= speed;
            *breal_point_lvl = BreakPointLvl(5);
        }
    }
}

pub fn despawn_enemies(
    mut commands: Commands,
    mut enemies: Query<(&Transform, Entity), With<Enemy>>,
    mut lifes: ResMut<Lifes>,
) {
    for (enemy_transform, entity) in &mut enemies {
        let translation = enemy_transform.translation;
        if translation.y <= BREAK_POINTS[5].y {
            commands.entity(entity).despawn();
            lifes.0 = lifes.0.saturating_sub(1);
        }
    }
}

pub fn wave_control(
    time: Res<Time>,
    mut wave_control: ResMut<WaveControl>,
    enemies: Query<Entity, With<Enemy>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let current_wave_enemies: usize = enemies.iter().len();

    if wave_control.spawned_count_in_wave == MAX_ENEMIES_PER_WAVE && current_wave_enemies == 0 {
        if wave_control.time_between_waves.paused() {
            wave_control.time_between_waves.unpause();
            wave_control.time_between_waves.reset();
            game_state.set(GameState::Building);
        }

        wave_control.time_between_waves.tick(time.delta());
        if wave_control.time_between_waves.just_finished() {
            wave_control.spawned_count_in_wave = 0;
            wave_control.wave_count += 1;
            game_state.set(GameState::Attacking);
            info!(
                "cooldown finished, starting Wave: {:?}",
                wave_control.wave_count
            );
        }
    }
}
