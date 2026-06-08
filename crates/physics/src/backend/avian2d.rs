use avian2d::prelude::{PhysicsDebugPlugin as AvianPhysicsDebugPlugin, PhysicsPlugins};
use bevy::prelude::*;

pub fn add_physics_backend(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default());
}

pub fn add_debug_backend(app: &mut App) {
    app.add_plugins(AvianPhysicsDebugPlugin::default());
}
