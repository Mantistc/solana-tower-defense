use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

pub struct TowerDefenseTilemapPlugin;

impl Plugin for TowerDefenseTilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_handle: Handle<TiledMap> = asset_server.load("tilemaps/tower_defense_tilemap.tmx");

    // Spawn the map with default options
    commands.spawn((
        TiledMapHandle(map_handle),
        TiledMapSettings {
            layer_positioning: LayerPositioning::Centered,
            ..default()
        },
    ));
}
