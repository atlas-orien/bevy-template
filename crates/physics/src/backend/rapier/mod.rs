mod dim2;
mod dim3;

use bevy::prelude::*;

pub fn add_physics_backend(app: &mut App) {
    dim2::add_physics_backend(app);
    dim3::add_physics_backend(app);
}

pub fn add_debug_backend(app: &mut App) {
    dim2::add_debug_backend(app);
    dim3::add_debug_backend(app);
}
