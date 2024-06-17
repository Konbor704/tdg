use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub hp: f32,
    pub max_hp: f32,
}

#[derive(Component)]
pub struct Damage {
    pub dmg: f32,
}

#[derive(Component)]
pub struct TextLabel;
