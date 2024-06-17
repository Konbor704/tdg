use bevy::prelude::*;

// Components
#[derive(Component)]
pub struct Enemy;

// Constants
pub const ENEMY_COLOR: Color = Color::Rgba {
    red: 0.7,
    green: 0.3,
    blue: 0.6,
    alpha: 1.0,
};
