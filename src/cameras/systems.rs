use bevy::prelude::*;

use crate::cameras::components::Camera;

pub fn main_c_spawn(mut c: Commands) {
    c.spawn((Camera2dBundle::default(), Camera));
}
