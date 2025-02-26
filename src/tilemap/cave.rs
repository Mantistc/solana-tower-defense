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

    let scale = 3.0;
    let tile_size = 16.0 * scale;
    let cave_width = 30;
    let start_y = 50.0;
    let mut rng = rand::rng();

    // define tile indices, last ones represent floor
    let tile_indices = vec![0, 2, 4, 6, 8, 8, 8, 8, 8, 8];

    for x in 0..cave_width {
        let x_position = tile_size * x as f32;

        for (y, &tile_index) in tile_indices.iter().enumerate() {
            let variant = if tile_index < 4 {
                tile_index
            } else if tile_index < 8 {
                tile_index + rng.random_range(0..2)
            } else {
                8 + rng.random_range(0..2)
            };

            let y_position = start_y - (tile_size * y as f32);
            let is_solid = tile_index < 8;

            // spawn main cave tiles
            let mut entity = commands.spawn((
                Sprite::from_atlas_image(
                    texture.clone(),
                    TextureAtlas {
                        layout: texture_atlas_layout.clone(),
                        index: variant,
                    },
                ),
                CaveTiles(if tile_index < 8 {
                    CaveTileType::Wall
                } else {
                    CaveTileType::Floor
                }),
                Transform {
                    translation: Vec3::new(x_position, y_position, 0.5),
                    scale: Vec3::splat(scale),
                    ..default()
                },
            ));

            if is_solid {
                entity.insert(Collider);
            }

            // spawn left and right walls
            if x == 0 || x == cave_width - 1 {
                commands.spawn((
                    Sprite::from_atlas_image(
                        texture.clone(),
                        TextureAtlas {
                            layout: texture_atlas_layout.clone(),
                            index: 4 + rng.random_range(0..2), // wall variation
                        },
                    ),
                    CaveTiles(CaveTileType::Wall),
                    Collider, // walls should have collision
                    Transform {
                        translation: Vec3::new(
                            if x == 0 {
                                -tile_size
                            } else {
                                x_position + tile_size
                            },
                            y_position,
                            0.5,
                        ),
                        scale: Vec3::splat(scale),
                        ..default()
                    },
                ));
            }
        }
    }

    // spawn bottom walls
    for x in 0..cave_width {
        let x_position = tile_size * x as f32;
        let y_position = start_y - (tile_size * tile_indices.len() as f32);

        commands.spawn((
            Sprite::from_atlas_image(
                texture.clone(),
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: 4 + rng.random_range(0..2), // wall variation
                },
            ),
            CaveTiles(CaveTileType::Wall),
            Collider, // bottom should have collision
            Transform {
                translation: Vec3::new(x_position, y_position, 0.5),
                scale: Vec3::splat(scale),
                ..default()
            },
        ));
    }
}
