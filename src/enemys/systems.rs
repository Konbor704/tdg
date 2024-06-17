use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::components::Health;
use crate::enemys::components::{Enemy, ENEMY_COLOR};

pub fn spawn_enemy(
    mut c: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Mesh2dHandle(mesh.add(Capsule2d::new(10.0, 20.0)));

    c.spawn((
        MaterialMesh2dBundle {
            mesh: shape,
            material: material.add(ENEMY_COLOR),
            transform: Transform::from_xyz(50., 0., 0.),
            ..Default::default()
        },
        Enemy,
        Health {
            hp: 100.0,
            max_hp: 100.0,
        },
    ));
}
