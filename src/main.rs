use bevy::{app::PluginGroupBuilder, prelude::*};
use camera::CameraPlugin;
use player::PlayerPlugin;
use tilemap::{configs::{SCREEN_HEIGHT, SCREEN_WIDTH}, CaveTileMapPlugin};
mod camera;
mod tilemap;
mod player;
mod animations;

fn main() {
    App::new()
        .add_plugins(default_pluggins())
        // tilemap plugins
        .add_plugins((CameraPlugin, PlayerPlugin))
        .add_plugins(CaveTileMapPlugin)
        // world inspector plugin to check/change and test stuff in runtime
        // .add_plugins(
        //     WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Space)),
        // )
        .run();
}

// default stuff
fn default_pluggins() -> PluginGroupBuilder {
    DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "Veralt".into(),
                resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        })
}
