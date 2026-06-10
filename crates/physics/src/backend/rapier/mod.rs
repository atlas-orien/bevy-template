mod dim2;
mod dim3;

use bevy::prelude::*;

pub use dim2::PhysicsQuery2d;
pub use dim3::PhysicsQuery3d;

pub fn add_physics_backend(app: &mut App) {
    dim2::add_physics_backend(app);
    dim3::add_physics_backend(app);
}

pub fn add_debug_backend(app: &mut App) {
    dim2::add_debug_backend(app);
    dim3::add_debug_backend(app);
}
