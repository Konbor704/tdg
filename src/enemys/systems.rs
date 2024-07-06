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

    c.spawn((
        MaterialMesh2dBundle {
            mesh: shape,
            material: material.add(ENEMY_COLOR),
            transform: Transform::from_xyz(50., 0., 0.),
            ..Default::default()
        },
        Enemy,
        BasicEnemy,
        Health {
            hp: 100.0,
            max_hp: 100.0,
        },
    ));
}

pub fn moving(
    mut q_moving: Query<(&mut Transform, &Enemy), (With<BasicEnemy>, Without<Castle>)>,
    time: Res<Time>,
    q_castle: Query<&Transform, With<Castle>>,
) {
    let castle_pos = match q_castle.get_single() {
        Ok(r) => r.translation,
        Err(_) => return,
    };

    for (mut transform, _enemy) in &mut q_moving {
        let dir = (castle_pos - transform.translation)
            .truncate()
            .normalize_or_zero()
            .extend(0.0);
        transform.translation += dir * MOVE_SPEED * time.delta_seconds();
    }
}
