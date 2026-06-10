mod convert;
mod systems;

use bevy::prelude::*;
use bevy_rapier2d::prelude::{NoUserData, RapierDebugRenderPlugin, RapierPhysicsPlugin};

pub fn add_physics_backend(app: &mut App) {
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_systems(
            Update,
            (
                systems::sync_physics_rigid_bodies,
                systems::sync_physics_locked_axes,
                systems::sync_physics_gravity_scales,
                systems::sync_physics_damping,
                systems::sync_physics_ccd,
                systems::sync_physics_soft_ccd,
                systems::sync_physics_sleeping,
                systems::sync_physics_rigid_body_disabled,
                systems::sync_physics_solver_iterations,
            ),
        )
        .add_systems(
            Update,
            (
                systems::sync_physics_colliders,
                systems::sync_physics_collider_disabled,
                systems::sync_physics_sensors,
                systems::sync_physics_materials,
                systems::sync_physics_masses,
                systems::sync_physics_collision_groups,
                systems::sync_physics_solver_groups,
                systems::sync_physics_active_events,
                systems::sync_physics_active_collision_types,
                systems::sync_physics_contact_skin,
                systems::sync_physics_contact_force_threshold,
            ),
        )
        .add_systems(
            Update,
            (
                systems::sync_physics_velocities,
                systems::sync_physics_angular_velocities,
                systems::sync_physics_forces,
                systems::sync_physics_impulses,
            ),
        );
}

pub fn add_debug_backend(app: &mut App) {
    app.add_plugins(RapierDebugRenderPlugin::default());
}
