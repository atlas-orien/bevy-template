use bevy::prelude::*;

use super::rapier;

pub fn add_physics_backend(app: &mut App) {
    rapier::add_physics_backend(app);
}

pub fn add_debug_backend(app: &mut App) {
    rapier::add_debug_backend(app);
}
