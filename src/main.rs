use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use enemies::EnemiesPlugins;
use tilemap::{
    configs::{SCREEN_HEIGHT, SCREEN_WIDTH},
    TowerDefenseTilemapPlugin,
};
mod animations;
mod enemies;
mod tilemap;

fn main() {
    App::new()
        .add_plugins(default_pluggins())
        .add_plugins(TilemapPlugin)
        .add_plugins(TiledMapPlugin::default())
        .add_plugins(TowerDefenseTilemapPlugin)
        .add_plugins(EnemiesPlugins)
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
