use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct AnimateSprite {
    pub first: usize,
    pub last: usize,
    pub timer: Timer,
    pub sprite_texture_atlas: Option<SpriteTextureAtlas>,
}

#[derive(Component, Clone, Deref, DerefMut)]
pub struct SpriteTextureAtlas((TextureAtlas, Handle<Image>));

pub fn set_texture_atlas(
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    path: &str,
    tile_size: UVec2,
    columns: u32,
    rows: u32,
    index: usize,
) -> SpriteTextureAtlas {
    let texture = asset_server.load(path);
    let layout = TextureAtlasLayout::from_grid(tile_size, columns, rows, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    SpriteTextureAtlas((
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index,
        },
        texture,
    ))
}
