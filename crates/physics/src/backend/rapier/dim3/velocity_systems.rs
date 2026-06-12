use bevy::prelude::*;
use bevy_rapier3d::prelude::Velocity as RapierVelocity;

use crate::{PhysicsAngularVelocity3d, PhysicsVelocity3d};

use super::convert;

type Synced<T> = Or<(Added<T>, Changed<T>)>;

pub fn sync_physics_velocities(
    mut commands: Commands,
    velocities: Query<
        (Entity, &PhysicsVelocity3d, Option<&RapierVelocity>),
        Synced<PhysicsVelocity3d>,
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
        (Entity, &PhysicsAngularVelocity3d, Option<&RapierVelocity>),
        Synced<PhysicsAngularVelocity3d>,
    >,
) {
    for (entity, velocity, current) in &angular_velocities {
        let mut next = current
            .copied()
            .unwrap_or_else(|| convert::angular_velocity(*velocity));
        next.angular = velocity.0;
        commands.entity(entity).insert(next);
    }
}
