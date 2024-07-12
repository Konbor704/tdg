use avian2d::prelude::*;
use bevy::prelude::*;

mod cameras;
mod components;
mod enemys;
mod events;
mod menu;
mod resources;
mod road;
mod systems;
mod tower;

use cameras::CamerasPlugin;
use enemys::EnemyPlugin;
use tower::TowerPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Tower Defence Game".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    }));

    app.add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity::ZERO);

    app.add_plugins((CamerasPlugin, TowerPlugin, EnemyPlugin));

    app.run();
}
