use avian2d::prelude::{Collider, Restitution};
use avian2d::prelude::{RigidBody, SweptCcd};
use bevy::{
    color::palettes::basic::{BLUE, GREEN, RED},
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::tower::components::Castle;

use crate::components::{Health, TextLabel};

use super::components::{CASTLE_COLOR, CASTLE_R};

pub fn spawn_tower(
    mut c: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Mesh2dHandle(mesh.add(Circle { radius: CASTLE_R }));

    c.spawn((
        MaterialMesh2dBundle {
            mesh: shape,
            material: material.add(CASTLE_COLOR),
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        },
        Castle,
        Health {
            hp: 100.0,
            max_hp: 100.0,
        },
        RigidBody::Kinematic,
        Collider::circle(10.0),
        SweptCcd::default(),
        Restitution::new(0.1),
    ));
}

pub fn damage_dealing_test(
    input: Res<ButtonInput<KeyCode>>,
    mut q: Query<&mut Health, With<Castle>>,
) {
    if input.just_pressed(KeyCode::Space) {
        for mut health in q.iter_mut() {
            health.hp -= 10.0;
        }
    }
}

pub fn hp_label(mut c: Commands) {
    c.spawn((
        TextLabel,
        TextBundle::from_sections([
            TextSection::from_style(TextStyle {
                font_size: 40.0,
                color: Color::from(GREEN),
                ..default()
            }),
            TextSection::new(
                " / ",
                TextStyle {
                    font_size: 40.0,
                    color: Color::from(BLUE),
                    ..Default::default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 40.0,
                color: Color::from(RED),
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            ..default()
        }),
    ));
}

pub fn update_hp(mut q: Query<&mut Text, With<TextLabel>>, hp_q: Query<&Health, With<Castle>>) {
    for health in hp_q.iter() {
        let hp = health.hp;
        let max = health.max_hp;
        let mut text = q.single_mut();
        text.sections[0].value = hp.to_string();
        text.sections[2].value = max.to_string();
    }
}
