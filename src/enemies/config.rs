use bevy::prelude::*;

pub const TOTAL_WAVES: u8 = 30;
pub const MAX_ENEMIES_PER_WAVE: u8 = 50;

#[derive(Resource)]
pub struct WaveControl {
    pub wave_count: u8,
    pub time_between_spawns: Timer,
    pub textures: Vec<(Handle<Image>, Handle<TextureAtlasLayout>)>,
}

pub fn load_enemy_sprites(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
) {
    let mut textures: Vec<(Handle<Image>, Handle<TextureAtlasLayout>)> = Vec::new();

    let enemy_list = vec!["orcs.png", "goblin.png", "skeleton.png", "demon.png"];

    for path in enemy_list {
        let texture = asset_server.load(path);
        let texture_atlas = TextureAtlasLayout::from_grid(UVec2::splat(48), 4, 4, None, None);
        let atlas_handle = texture_atlas_layouts.add(texture_atlas);

        textures.push((texture, atlas_handle));
    }

    commands.insert_resource(WaveControl {
        textures,
        wave_count: 0,
        time_between_spawns: Timer::from_seconds(1.5, TimerMode::Repeating),
    });
}
