use bevy::prelude::*;

mod components;
mod events;
mod resources;
mod systems;

pub struct TowerPlugin;

use systems::{damage_dealing_test, hp_label, spawn_tower, update_hp};

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_tower, hp_label));
        app.add_systems(Update, (damage_dealing_test, update_hp));
    }
}
