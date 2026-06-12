use bevy::prelude::*;
use bevy_rapier3d::prelude::{
    NoUserData, PhysicsSet as RapierPhysicsSet, RapierDebugRenderPlugin, RapierPhysicsPlugin,
};

use super::{events, systems, velocity_systems};

pub fn add_physics_backend(app: &mut App) {
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
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
                velocity_systems::sync_physics_velocities,
                velocity_systems::sync_physics_angular_velocities,
                systems::sync_physics_forces,
                systems::sync_physics_impulses,
                systems::sync_physics_impulse_joints,
                systems::sync_physics_character_controllers,
            ),
        )
        .add_systems(
            PostUpdate,
            (
                events::forward_collision_events,
                events::forward_contact_force_events,
                systems::sync_physics_character_controller_outputs,
            )
                .after(RapierPhysicsSet::Writeback),
        );
}

pub fn add_debug_backend(app: &mut App) {
    app.add_plugins(RapierDebugRenderPlugin::default());
}
