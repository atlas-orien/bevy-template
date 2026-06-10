use bevy::prelude::*;
use bevy_rapier2d::prelude::{
    ColliderDisabled as RapierColliderDisabled, Sensor as RapierSensor, Velocity as RapierVelocity,
};

use crate::{
    PhysicsActiveCollisionTypes, PhysicsActiveEvents, PhysicsAdditionalSolverIterations,
    PhysicsAngularVelocity2d, PhysicsCcd, PhysicsCollider2d, PhysicsColliderDisabled,
    PhysicsCollisionGroups, PhysicsContactForceEventThreshold, PhysicsContactSkin, PhysicsDamping,
    PhysicsForce2d, PhysicsGravityScale, PhysicsImpulse2d, PhysicsImpulseJoint2d,
    PhysicsLockedAxes, PhysicsMass, PhysicsMaterial, PhysicsRigidBody, PhysicsRigidBodyDisabled,
    PhysicsSensor, PhysicsSleeping, PhysicsSoftCcd, PhysicsSolverGroups, PhysicsVelocity2d,
};

use super::convert;

type Synced<T> = Or<(Added<T>, Changed<T>)>;
type RigidBodySynced = Or<(
    Added<PhysicsRigidBody>,
    Changed<PhysicsRigidBody>,
    Added<PhysicsCollider2d>,
    Changed<PhysicsCollider2d>,
)>;

fn has_collider(entity: Entity, colliders: &Query<&PhysicsCollider2d>) -> bool {
    colliders.get(entity).is_ok()
}

pub fn sync_physics_rigid_bodies(
    mut commands: Commands,
    rigid_bodies: Query<(Entity, &PhysicsRigidBody, &PhysicsCollider2d), RigidBodySynced>,
) {
    for (entity, rigid_body, _) in &rigid_bodies {
        commands
            .entity(entity)
            .insert(convert::rigid_body(*rigid_body));
    }
}

pub fn sync_physics_locked_axes(
    mut commands: Commands,
    locked_axes: Query<(Entity, &PhysicsLockedAxes), Synced<PhysicsLockedAxes>>,
    colliders: Query<&PhysicsCollider2d>,
) {
    for (entity, locked_axes) in &locked_axes {
        if !has_collider(entity, &colliders) {
            continue;
        }
        commands
            .entity(entity)
            .insert(convert::locked_axes(*locked_axes));
    }
}

pub fn sync_physics_gravity_scales(
    mut commands: Commands,
    gravity_scales: Query<(Entity, &PhysicsGravityScale), Synced<PhysicsGravityScale>>,
    colliders: Query<&PhysicsCollider2d>,
) {
    for (entity, gravity_scale) in &gravity_scales {
        if !has_collider(entity, &colliders) {
            continue;
        }
        commands
            .entity(entity)
            .insert(convert::gravity_scale(*gravity_scale));
    }
}

pub fn sync_physics_damping(
    mut commands: Commands,
    damping: Query<(Entity, &PhysicsDamping), Synced<PhysicsDamping>>,
    colliders: Query<&PhysicsCollider2d>,
) {
    for (entity, damping) in &damping {
        if !has_collider(entity, &colliders) {
            continue;
        }
        commands.entity(entity).insert(convert::damping(*damping));
    }
}

pub fn sync_physics_ccd(
    mut commands: Commands,
    ccd: Query<(Entity, &PhysicsCcd), Synced<PhysicsCcd>>,
    colliders: Query<&PhysicsCollider2d>,
) {
    for (entity, ccd) in &ccd {
        if !has_collider(entity, &colliders) {
            continue;
        }
        commands.entity(entity).insert(convert::ccd(*ccd));
    }
}

pub fn sync_physics_soft_ccd(
    mut commands: Commands,
    soft_ccd: Query<(Entity, &PhysicsSoftCcd), Synced<PhysicsSoftCcd>>,
    colliders: Query<&PhysicsCollider2d>,
) {
    for (entity, soft_ccd) in &soft_ccd {
        if !has_collider(entity, &colliders) {
            continue;
        }
        commands.entity(entity).insert(convert::soft_ccd(*soft_ccd));
    }
}

pub fn sync_physics_sleeping(
    mut commands: Commands,
    sleeping: Query<(Entity, &PhysicsSleeping), Synced<PhysicsSleeping>>,
    colliders: Query<&PhysicsCollider2d>,
) {
    for (entity, sleeping) in &sleeping {
        if !has_collider(entity, &colliders) {
            continue;
        }
        commands.entity(entity).insert(convert::sleeping(*sleeping));
    }
}

pub fn sync_physics_rigid_body_disabled(
    mut commands: Commands,
    disabled: Query<Entity, Added<PhysicsRigidBodyDisabled>>,
    colliders: Query<&PhysicsCollider2d>,
) {
    for entity in &disabled {
        if !has_collider(entity, &colliders) {
            continue;
        }
        commands
            .entity(entity)
            .insert(convert::rigid_body_disabled());
    }
}

pub fn sync_physics_solver_iterations(
    mut commands: Commands,
    iterations: Query<
        (Entity, &PhysicsAdditionalSolverIterations),
        Synced<PhysicsAdditionalSolverIterations>,
    >,
    colliders: Query<&PhysicsCollider2d>,
) {
    for (entity, iterations) in &iterations {
        if !has_collider(entity, &colliders) {
            continue;
        }
        commands
            .entity(entity)
            .insert(convert::solver_iterations(*iterations));
    }
}

pub fn sync_physics_colliders(
    mut commands: Commands,
    colliders: Query<(Entity, &PhysicsCollider2d), Synced<PhysicsCollider2d>>,
) {
    for (entity, collider) in &colliders {
        let Some(collider) = convert::collider(collider) else {
            continue;
        };
        commands.entity(entity).insert(collider);
    }
}

pub fn sync_physics_collider_disabled(
    mut commands: Commands,
    disabled: Query<Entity, Added<PhysicsColliderDisabled>>,
    colliders: Query<&PhysicsCollider2d>,
) {
    for entity in &disabled {
        if !has_collider(entity, &colliders) {
            continue;
        }
        commands.entity(entity).insert(RapierColliderDisabled);
    }
}

pub fn sync_physics_sensors(
    mut commands: Commands,
    sensors: Query<Entity, Added<PhysicsSensor>>,
    colliders: Query<&PhysicsCollider2d>,
) {
    for entity in &sensors {
        let Ok(_) = colliders.get(entity) else {
            continue;
        };
        commands.entity(entity).insert(RapierSensor);
    }
}

pub fn sync_physics_materials(
    mut commands: Commands,
    materials: Query<(Entity, &PhysicsMaterial), Synced<PhysicsMaterial>>,
    colliders: Query<&PhysicsCollider2d>,
) {
    for (entity, material) in &materials {
        let Ok(_) = colliders.get(entity) else {
            continue;
        };
        commands.entity(entity).insert(convert::material(*material));
    }
}

pub fn sync_physics_masses(
    mut commands: Commands,
    masses: Query<(Entity, &PhysicsMass), Synced<PhysicsMass>>,
    colliders: Query<&PhysicsCollider2d>,
) {
    for (entity, mass) in &masses {
        let Ok(_) = colliders.get(entity) else {
            continue;
        };
        commands.entity(entity).insert(convert::mass(*mass));
    }
}

pub fn sync_physics_forces(
    mut commands: Commands,
    forces: Query<(Entity, &PhysicsForce2d), Synced<PhysicsForce2d>>,
) {
    for (entity, force) in &forces {
        commands.entity(entity).insert(convert::force(*force));
    }
}

pub fn sync_physics_impulses(
    mut commands: Commands,
    impulses: Query<(Entity, &PhysicsImpulse2d), Synced<PhysicsImpulse2d>>,
) {
    for (entity, impulse) in &impulses {
        commands.entity(entity).insert(convert::impulse(*impulse));
    }
}

pub fn sync_physics_impulse_joints(
    mut commands: Commands,
    joints: Query<(Entity, &PhysicsImpulseJoint2d), Synced<PhysicsImpulseJoint2d>>,
) {
    for (entity, joint) in &joints {
        commands
            .entity(entity)
            .insert(convert::impulse_joint(*joint));
    }
}

pub fn sync_physics_collision_groups(
    mut commands: Commands,
    groups: Query<(Entity, &PhysicsCollisionGroups), Synced<PhysicsCollisionGroups>>,
    colliders: Query<&PhysicsCollider2d>,
) {
    for (entity, groups) in &groups {
        if !has_collider(entity, &colliders) {
            continue;
        }
        let Some(groups) = convert::collision_groups(*groups) else {
            continue;
        };
        commands.entity(entity).insert(groups);
    }
}

pub fn sync_physics_solver_groups(
    mut commands: Commands,
    groups: Query<(Entity, &PhysicsSolverGroups), Synced<PhysicsSolverGroups>>,
    colliders: Query<&PhysicsCollider2d>,
) {
    for (entity, groups) in &groups {
        if !has_collider(entity, &colliders) {
            continue;
        }
        let Some(groups) = convert::solver_groups(*groups) else {
            continue;
        };
        commands.entity(entity).insert(groups);
    }
}

pub fn sync_physics_active_events(
    mut commands: Commands,
    events: Query<(Entity, &PhysicsActiveEvents), Synced<PhysicsActiveEvents>>,
    colliders: Query<&PhysicsCollider2d>,
) {
    for (entity, events) in &events {
        if !has_collider(entity, &colliders) {
            continue;
        }
        commands
            .entity(entity)
            .insert(convert::active_events(*events));
    }
}

pub fn sync_physics_active_collision_types(
    mut commands: Commands,
    collision_types: Query<
        (Entity, &PhysicsActiveCollisionTypes),
        Synced<PhysicsActiveCollisionTypes>,
    >,
    colliders: Query<&PhysicsCollider2d>,
) {
    for (entity, collision_types) in &collision_types {
        if !has_collider(entity, &colliders) {
            continue;
        }
        commands
            .entity(entity)
            .insert(convert::active_collision_types(*collision_types));
    }
}

pub fn sync_physics_contact_skin(
    mut commands: Commands,
    contact_skin: Query<(Entity, &PhysicsContactSkin), Synced<PhysicsContactSkin>>,
    colliders: Query<&PhysicsCollider2d>,
) {
    for (entity, contact_skin) in &contact_skin {
        if !has_collider(entity, &colliders) {
            continue;
        }
        commands
            .entity(entity)
            .insert(convert::contact_skin(*contact_skin));
    }
}

pub fn sync_physics_contact_force_threshold(
    mut commands: Commands,
    thresholds: Query<
        (Entity, &PhysicsContactForceEventThreshold),
        Synced<PhysicsContactForceEventThreshold>,
    >,
    colliders: Query<&PhysicsCollider2d>,
) {
    for (entity, threshold) in &thresholds {
        if !has_collider(entity, &colliders) {
            continue;
        }
        commands
            .entity(entity)
            .insert(convert::contact_force_threshold(*threshold));
    }
}

pub fn sync_physics_velocities(
    mut commands: Commands,
    velocities: Query<
        (Entity, &PhysicsVelocity2d, Option<&RapierVelocity>),
        Synced<PhysicsVelocity2d>,
    >,
) {
    for (entity, velocity, current) in &velocities {
        let mut next = current
            .copied()
            .unwrap_or_else(|| convert::linear_velocity(*velocity));
        next.linear = velocity.0;
        commands.entity(entity).insert(next);
    }
}

pub fn sync_physics_angular_velocities(
    mut commands: Commands,
    angular_velocities: Query<
        (Entity, &PhysicsAngularVelocity2d, Option<&RapierVelocity>),
        Synced<PhysicsAngularVelocity2d>,
    >,
) {
    for (entity, velocity, current) in &angular_velocities {
        let mut next = current
            .copied()
            .unwrap_or_else(|| convert::angular_velocity(*velocity));
        next.angular = velocity.radians_per_second;
        commands.entity(entity).insert(next);
    }
}
