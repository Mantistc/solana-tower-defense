use bevy::prelude::*;
use rand::Rng;

use super::Collider;

#[derive(Component)]
pub struct CaveTiles(CaveTileType);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaveTileType {
    Floor,
    Wall,
}

pub struct CaveTileMapPlugin;

impl Plugin for CaveTileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cave_tile_map);
    }
}

pub fn spawn_cave_tile_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("tilemap_sprite_sheet.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 2, 5, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let scale = 3f32;
    let tile_width = 16.0 * scale;
    let start_y = 50.0;

    let tile_indices = vec![0, 2, 4, 6, 8, 8, 8, 8, 8, 8, 8];
    let mut rng = rand::rng();
    for i in 0..30 {
        let x_position = tile_width * i as f32 + 1.0;
        for (j, &tile_index) in tile_indices.iter().enumerate() {
            let variant = if tile_index < 4 {
                tile_index
            } else if tile_index < 8 {
                tile_index + rng.random_range(0..2)
            } else {
                8 + rng.random_range(0..2)
            };
            let y_position = start_y - (tile_width * j as f32);
            let is_solid = tile_index < 8;

            let mut entity = commands.spawn((
                Sprite::from_atlas_image(
                    texture.clone(),
                    TextureAtlas {
                        layout: texture_atlas_layout.clone(),
                        index: variant,
                    },
                ),
                CaveTiles(CaveTileType::Wall),
                Transform {
                    translation: Vec3::new(x_position, y_position, 0.5),
                    scale: Vec3::splat(scale),
                    ..default()
                },
            ));

            if is_solid {
                entity.insert(Collider);
            }
        }
    }
}
