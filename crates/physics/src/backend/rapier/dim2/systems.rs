use bevy::prelude::*;
use bevy_rapier2d::prelude::{Sensor as RapierSensor, Velocity as RapierVelocity};

use crate::{
    PhysicsAngularVelocity2d, PhysicsBody, PhysicsCollider, PhysicsMass, PhysicsMaterial,
    PhysicsSensor, PhysicsVelocity2d,
};

use super::convert;

type Synced<T> = Or<(Added<T>, Changed<T>)>;
type BodySynced = Or<(
    Added<PhysicsBody>,
    Changed<PhysicsBody>,
    Added<PhysicsCollider>,
    Changed<PhysicsCollider>,
)>;

pub fn sync_physics_bodies(
    mut commands: Commands,
    bodies: Query<(Entity, &PhysicsBody, &PhysicsCollider), BodySynced>,
) {
    for (entity, body, collider) in &bodies {
        if !collider.is_2d() {
            continue;
        }
        commands.entity(entity).insert(convert::body(*body));
    }
}

pub fn sync_physics_colliders(
    mut commands: Commands,
    colliders: Query<(Entity, &PhysicsCollider), Synced<PhysicsCollider>>,
) {
    for (entity, collider) in &colliders {
        if !collider.is_2d() {
            continue;
        }
        commands.entity(entity).insert(convert::collider(*collider));
    }
}

pub fn sync_physics_sensors(
    mut commands: Commands,
    sensors: Query<Entity, Added<PhysicsSensor>>,
    colliders: Query<&PhysicsCollider>,
) {
    for entity in &sensors {
        let Ok(collider) = colliders.get(entity) else {
            continue;
        };
        if !collider.is_2d() {
            continue;
        }
        commands.entity(entity).insert(RapierSensor);
    }
}

pub fn sync_physics_materials(
    mut commands: Commands,
    materials: Query<(Entity, &PhysicsMaterial), Synced<PhysicsMaterial>>,
    colliders: Query<&PhysicsCollider>,
) {
    for (entity, material) in &materials {
        let Ok(collider) = colliders.get(entity) else {
            continue;
        };
        if !collider.is_2d() {
            continue;
        }
        commands.entity(entity).insert(convert::material(*material));
    }
}

pub fn sync_physics_masses(
    mut commands: Commands,
    masses: Query<(Entity, &PhysicsMass), Synced<PhysicsMass>>,
    colliders: Query<&PhysicsCollider>,
) {
    for (entity, mass) in &masses {
        let Ok(collider) = colliders.get(entity) else {
            continue;
        };
        if !collider.is_2d() {
            continue;
        }
        commands.entity(entity).insert(convert::mass(*mass));
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
