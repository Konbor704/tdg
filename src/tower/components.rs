use bevy::prelude::*;

// COMPONENTS
#[derive(Component)]
pub struct Castle;

// CONSTANTS
pub const CASTLE_R: f32 = 10.0;
pub const CASTLE_COLOR: Color = Color::Rgba {
    red: 1.,
    green: 1.,
    blue: 1.,
    alpha: 1.0,
};
