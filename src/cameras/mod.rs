use bevy::prelude::*;

mod components;
mod events;
mod resources;
mod systems;

use systems::main_c_spawn;

pub struct CamerasPlugin;

impl Plugin for CamerasPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, main_c_spawn);
    }
}
