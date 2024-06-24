use bevy::prelude::*;

mod components;
mod events;
mod resources;
mod systems;

use crate::enemys::systems::{moving, spawn_enemy};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemy);
        app.add_systems(Update, moving);
    }
}
