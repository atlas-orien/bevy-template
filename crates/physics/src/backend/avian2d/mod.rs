mod convert;
mod systems;

use avian2d::prelude::{
    PhysicsDebugPlugin as AvianPhysicsDebugPlugin, PhysicsPlugins as AvianPhysicsPlugins,
};
use bevy::prelude::*;

pub fn add_physics_backend(app: &mut App) {
    app.add_plugins(AvianPhysicsPlugins::default())
        .add_systems(
            Update,
            (
                systems::sync_physics_bodies,
                systems::sync_physics_colliders,
                systems::sync_physics_sensors,
                systems::sync_physics_materials,
                systems::sync_physics_masses,
                systems::sync_physics_velocities,
                systems::sync_physics_angular_velocities,
            ),
        );
}

pub fn add_debug_backend(app: &mut App) {
    app.add_plugins(AvianPhysicsDebugPlugin);
}
