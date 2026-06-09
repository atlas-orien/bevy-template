use avian2d::prelude::Sensor as AvianSensor;
use bevy::prelude::*;

use crate::{
    PhysicsAngularVelocity2d, PhysicsBody, PhysicsCollider, PhysicsMass, PhysicsMaterial,
    PhysicsSensor, PhysicsVelocity2d,
};

use super::convert;

pub fn sync_physics_bodies(
    mut commands: Commands,
    bodies: Query<(Entity, &PhysicsBody), Or<(Added<PhysicsBody>, Changed<PhysicsBody>)>>,
) {
    for (entity, body) in &bodies {
        commands.entity(entity).insert(convert::body(*body));
    }
}

pub fn sync_physics_colliders(
    mut commands: Commands,
    colliders: Query<
        (Entity, &PhysicsCollider),
        Or<(Added<PhysicsCollider>, Changed<PhysicsCollider>)>,
    >,
) {
    for (entity, collider) in &colliders {
        commands.entity(entity).insert(convert::collider(*collider));
    }
}

pub fn sync_physics_sensors(mut commands: Commands, sensors: Query<Entity, Added<PhysicsSensor>>) {
    for entity in &sensors {
        commands.entity(entity).insert(AvianSensor);
    }
}

pub fn sync_physics_materials(
    mut commands: Commands,
    materials: Query<
        (Entity, &PhysicsMaterial),
        Or<(Added<PhysicsMaterial>, Changed<PhysicsMaterial>)>,
    >,
) {
    for (entity, material) in &materials {
        commands.entity(entity).insert(convert::material(*material));
    }
}

pub fn sync_physics_masses(
    mut commands: Commands,
    masses: Query<(Entity, &PhysicsMass), Or<(Added<PhysicsMass>, Changed<PhysicsMass>)>>,
) {
    for (entity, mass) in &masses {
        commands.entity(entity).insert(convert::mass(*mass));
    }
}

pub fn sync_physics_velocities(
    mut commands: Commands,
    velocities: Query<
        (Entity, &PhysicsVelocity2d),
        Or<(Added<PhysicsVelocity2d>, Changed<PhysicsVelocity2d>)>,
    >,
) {
    for (entity, velocity) in &velocities {
        commands.entity(entity).insert(convert::velocity(*velocity));
    }
}

pub fn sync_physics_angular_velocities(
    mut commands: Commands,
    angular_velocities: Query<
        (Entity, &PhysicsAngularVelocity2d),
        Or<(
            Added<PhysicsAngularVelocity2d>,
            Changed<PhysicsAngularVelocity2d>,
        )>,
    >,
) {
    for (entity, velocity) in &angular_velocities {
        commands
            .entity(entity)
            .insert(convert::angular_velocity(*velocity));
    }
}
