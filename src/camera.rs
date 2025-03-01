use crate::player::Player;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
            // .add_systems(Update, follow_player);
    }
}

#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        MainCamera,
    ));
}

fn _follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            let camera_speed = 5.0;
            let delta = time.delta_secs();

            let target_position = player_transform.translation;

            let smooth_position = camera_transform.translation.lerp(target_position, camera_speed * delta);

            camera_transform.translation = Vec3::new(
                smooth_position.x.round(),
                smooth_position.y.round(),
                camera_transform.translation.z,
            );
        }
    }
}