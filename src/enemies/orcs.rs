use bevy::prelude::*;

use super::Enemy;

// define plugin
pub struct OrcsPlugin;

impl Plugin for OrcsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_orcs)
            .add_systems(Update, attack);
    }
}

// define components

#[derive(Component)]
pub struct Orcs {
    pub life: u8,
    pub attack_damage: u8,
    pub speed: f32,
    pub attack_cooldown: Timer,
}

impl Default for Orcs {
    fn default() -> Self {
        Self {
            life: 100,
            attack_damage: 10,
            speed: 65.0,
            attack_cooldown: Timer::from_seconds(1.5, TimerMode::Repeating),
        }
    }
}

// define systems
const SPAWN_AMOUNT: u8 = 10;
pub fn spawn_orcs(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("enemies/orcs/orc_idle.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(48, 32), 6, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    for i in 0..SPAWN_AMOUNT {
        commands.spawn((
            Sprite::from_atlas_image(
                texture.clone(),
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: 0,
                },
            ),
            Transform {
                translation: Vec3::new(150.0 * i as f32, -125.0, 1.0),
                scale: Vec3::splat(2.0),
                ..default()
            },
            Orcs::default(),
            Enemy,
        ));
    }
}

pub fn attack() {}
