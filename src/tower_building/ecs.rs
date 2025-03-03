use bevy::prelude::*;


pub struct TowersPlugin;

impl Plugin for TowersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
    }
}


#[derive(Component, Debug)]
pub struct Tower {
    pub life: u8,
    pub attack_damage: u8,
}


pub fn spawn(){}