use avian2d::prelude::{Collider, LinearVelocity, Restitution};
use avian2d::prelude::{RigidBody, SweptCcd};
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::components::Health;
use crate::enemys::components::{BasicEnemy, Enemy, ENEMY_COLOR, MOVE_SPEED};
use crate::tower::components::Castle;

pub fn spawn_enemy(
    mut c: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Mesh2dHandle(mesh.add(Capsule2d::new(10.0, 20.0)));

    let velicity: Vec2 = Vec2::new(-70.0, 0.0);

    c.spawn((
        MaterialMesh2dBundle {
            mesh: shape,
            material: material.add(ENEMY_COLOR),
            transform: Transform::from_xyz(450., 0., 0.),
            ..Default::default()
        },
        Enemy,
        BasicEnemy,
        Health {
            hp: 100.0,
            max_hp: 100.0,
        },
        RigidBody::Dynamic,
        Collider::capsule(10., 20.),
        SweptCcd::default(),
        LinearVelocity(velicity),
        Restitution::new(0.1),
    ));
}

pub fn moving(mut q: Query<&mut LinearVelocity, Without<Castle>>) {
    for mut linear_velocity in q.iter_mut() {
        linear_velocity.x -= 0.1;
    }
}

// pub fn moving(
//     mut q_moving: Query<(&mut Transform, &Enemy), (With<RigidBody>, Without<Castle>)>,
//     time: Res<Time>,
//     q_castle: Query<&Transform, With<Castle>>,
// ) {
//     let castle_pos = match q_castle.get_single() {
//         Ok(r) => r.translation,
//         Err(_) => return,
//     };
//
//     for (mut transform, _enemy) in q_moving.iter_mut() {
//         let dir = (castle_pos - transform.translation)
//             .truncate()
//             .normalize_or_zero()
//             .extend(0.0);
//         transform.translation += dir * MOVE_SPEED * time.delta_seconds();
//     }
// }
