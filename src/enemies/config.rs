use bevy::prelude::*;

use super::OrcsPlugin;

#[derive(Component)]
pub struct Enemy;

pub struct EnemiesPlugins;

impl Plugin for EnemiesPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(OrcsPlugin);
    }
}
