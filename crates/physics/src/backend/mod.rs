mod rapier;

use bevy::prelude::*;

pub use rapier::{PhysicsQuery2d, PhysicsQuery3d};

pub fn add_physics_backend(app: &mut App) {
    rapier::add_physics_backend(app);
}

pub fn add_debug_backend(app: &mut App) {
    rapier::add_debug_backend(app);
}
