mod convert;
mod systems;

use bevy::prelude::*;
use bevy_rapier2d::prelude::{NoUserData, RapierDebugRenderPlugin, RapierPhysicsPlugin};

pub fn add_physics_backend(app: &mut App) {
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
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
    app.add_plugins(RapierDebugRenderPlugin::default());
}
