use bevy::{app::PluginGroupBuilder, input::common_conditions::input_toggle_active, prelude::*};
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use enemies::EnemiesPlugin;
use solana::SolanaPlugin;
use tilemap::{
    configs::{SCREEN_HEIGHT, SCREEN_WIDTH},
    TowerDefenseTilemapPlugin,
};
use tower_building::TowersPlugin;
use ui::UiPlugin;
mod enemies;
mod solana;
mod tilemap;
mod tower_building;
mod ui;

fn main() {
    App::new()
        .add_plugins(default_pluggins())
        .add_plugins(TilemapPlugin)
        .add_plugins(TiledMapPlugin::default())
        .add_plugins(TowerDefenseTilemapPlugin)
        .add_plugins(SolanaPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(EnemiesPlugin)
        .add_plugins(TowersPlugin)
        // world inspector plugin to check/change and test stuff in runtime
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Space)),
        )
        .run();
}

fn default_pluggins() -> PluginGroupBuilder {
    DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "Solana Tower Defense".into(),
                resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        })
}


// Think of this like a .env
#[toml_cfg::toml_config]
pub struct Variables {
    #[default("")]
    sol_rpc: &'static str,
    #[default("")]
    payment_wallet: &'static str,
    #[default("")]
    signer_wallet_path: &'static str,
}