use bevy::prelude::*;

use crate::PhysicsConfig;
use crate::backend;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PhysicsConfig>();
        backend::add_physics_backend(app);
    }
}

pub struct PhysicsDebugPlugin;

impl Plugin for PhysicsDebugPlugin {
    fn build(&self, app: &mut App) {
        backend::add_debug_backend(app);
    }
}
