use bevy::prelude::*;

use crate::{
    animations::{animate_orcs, OrcsAnimation, OrcsAnimationState},
    tilemap::{Collider, COLLISION_THRESHOLD},
};

use super::Enemy;

// define plugin
pub struct OrcsPlugin;

impl Plugin for OrcsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_orcs)
            .add_systems(Update, (animate_orcs, move_orcs, avoid_colliders));
    }
}

// define components

#[derive(Component)]
pub struct Orcs {}

impl Default for Orcs {
    fn default() -> Self {
        Self {}
    }
}

// define systems
const SPAWN_AMOUNT: u8 = 10;
pub fn spawn_orcs(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("enemies/orcs/orc_sprite_sheet.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 8, 6, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let orcs_animation = OrcsAnimation::default();

    for i in 0..SPAWN_AMOUNT {
        commands.spawn((
            Sprite::from_atlas_image(
                texture.clone(),
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: orcs_animation.idle.first,
                },
            ),
            Transform {
                translation: Vec3::new(150.0 * i as f32, -125.0, 1.0),
                scale: Vec3::splat(2.0),
                ..default()
            },
            Orcs::default(),
            Enemy,
            orcs_animation.clone(),
        ));
    }
}

const ORC_SEPARATION: f32 = 45.0;

pub fn move_orcs(mut orcs: Query<(&mut Transform, &Orcs, &mut OrcsAnimation)>, time: Res<Time>) {}

pub fn avoid_colliders(
    mut orcs: Query<&mut Transform, (Without<Collider>, With<Orcs>)>,
    colliders: Query<&Transform, With<Collider>>,
) {
    let collider_positions: Vec<Vec2> =
        colliders.iter().map(|t| t.translation.truncate()).collect();
    for mut orc_transform in &mut orcs {
        let orc_position = orc_transform.translation.truncate();
        for collider_position in &collider_positions {
            let distance = orc_position.distance_squared(*collider_position);
            if distance < COLLISION_THRESHOLD {
                let direction = (orc_position - collider_position).normalize_or_zero();
                orc_transform.translation += direction.extend(0.0) + ORC_SEPARATION;
            }
        }
    }
}
