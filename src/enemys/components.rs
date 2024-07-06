use bevy::prelude::*;

// Components
#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct BasicEnemy;

// Constants
pub const ENEMY_COLOR: Color = Color::srgba(0.7, 0.3, 0.6, 1.0);

pub const MOVE_SPEED: f32 = 30.0;
