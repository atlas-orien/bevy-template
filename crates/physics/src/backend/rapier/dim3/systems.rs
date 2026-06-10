use bevy::prelude::*;
use bevy_rapier3d::prelude::{Sensor as RapierSensor, Velocity as RapierVelocity};

use crate::{
    PhysicsAngularVelocity3d, PhysicsCollider3d, PhysicsForce3d, PhysicsImpulse3d, PhysicsMass,
    PhysicsMaterial, PhysicsRigidBody, PhysicsSensor, PhysicsVelocity3d,
};

use super::convert;

type Synced<T> = Or<(Added<T>, Changed<T>)>;

pub fn sync_physics_rigid_bodies(
    mut commands: Commands,
    rigid_bodies: Query<(Entity, &PhysicsRigidBody), Synced<PhysicsRigidBody>>,
) {
    for (entity, rigid_body) in &rigid_bodies {
        commands
            .entity(entity)
            .insert(convert::rigid_body(*rigid_body));
    }
}

pub fn sync_physics_colliders(
    mut commands: Commands,
    colliders: Query<(Entity, &PhysicsCollider3d), Synced<PhysicsCollider3d>>,
) {
    for (entity, collider) in &colliders {
        let Some(collider) = convert::collider(collider) else {
            continue;
        };
        commands.entity(entity).insert(collider);
    }
}

pub fn sync_physics_sensors(
    mut commands: Commands,
    sensors: Query<Entity, Added<PhysicsSensor>>,
    colliders: Query<&PhysicsCollider3d>,
) {
    for entity in &sensors {
        if colliders.get(entity).is_ok() {
            commands.entity(entity).insert(RapierSensor);
        }
    }
}

pub fn sync_physics_materials(
    mut commands: Commands,
    materials: Query<(Entity, &PhysicsMaterial), Synced<PhysicsMaterial>>,
    colliders: Query<&PhysicsCollider3d>,
) {
    for (entity, material) in &materials {
        if colliders.get(entity).is_ok() {
            commands.entity(entity).insert(convert::material(*material));
        }
    }
}

pub fn sync_physics_masses(
    mut commands: Commands,
    masses: Query<(Entity, &PhysicsMass), Synced<PhysicsMass>>,
    colliders: Query<&PhysicsCollider3d>,
) {
    for (entity, mass) in &masses {
        if colliders.get(entity).is_ok() {
            commands.entity(entity).insert(convert::mass(*mass));
        }
    }
}

pub fn sync_physics_velocities(
    mut commands: Commands,
    velocities: Query<(Entity, &PhysicsVelocity3d), Synced<PhysicsVelocity3d>>,
    existing_velocities: Query<&RapierVelocity>,
) {
    for (entity, velocity) in &velocities {
        let mut velocity = convert::linear_velocity(*velocity);
        if let Ok(existing) = existing_velocities.get(entity) {
            velocity.angular = existing.angular;
        }
        commands.entity(entity).insert(velocity);
    }
}

pub fn sync_physics_angular_velocities(
    mut commands: Commands,
    velocities: Query<(Entity, &PhysicsAngularVelocity3d), Synced<PhysicsAngularVelocity3d>>,
    existing_velocities: Query<&RapierVelocity>,
) {
    for (entity, angular_velocity) in &velocities {
        let mut velocity = convert::angular_velocity(*angular_velocity);
        if let Ok(existing) = existing_velocities.get(entity) {
            velocity.linear = existing.linear;
        }
        commands.entity(entity).insert(velocity);
    }
}

pub fn sync_physics_forces(
    mut commands: Commands,
    forces: Query<(Entity, &PhysicsForce3d), Synced<PhysicsForce3d>>,
) {
    for (entity, force) in &forces {
        commands.entity(entity).insert(convert::force(*force));
    }
}

pub fn sync_physics_impulses(
    mut commands: Commands,
    impulses: Query<(Entity, &PhysicsImpulse3d), Synced<PhysicsImpulse3d>>,
) {
    for (entity, impulse) in &impulses {
        commands.entity(entity).insert(convert::impulse(*impulse));
    }
}
