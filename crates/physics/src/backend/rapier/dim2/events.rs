use bevy::prelude::*;
use bevy_rapier2d::{
    prelude::{
        CollisionEvent as RapierCollisionEvent, ContactForceEvent as RapierContactForceEvent,
    },
    rapier::geometry::CollisionEventFlags,
};

use crate::{
    PhysicsCollisionEnded, PhysicsCollisionStarted, PhysicsContactForce2d, PhysicsSensorMarker,
    PhysicsSensorTriggered,
};

pub fn forward_collision_events(
    mut rapier_events: MessageReader<RapierCollisionEvent>,
    sensors: Query<(), With<PhysicsSensorMarker>>,
    mut started_events: MessageWriter<PhysicsCollisionStarted>,
    mut ended_events: MessageWriter<PhysicsCollisionEnded>,
    mut sensor_events: MessageWriter<PhysicsSensorTriggered>,
) {
    for event in rapier_events.read() {
        match *event {
            RapierCollisionEvent::Started(a, b, flags) => {
                if flags.contains(CollisionEventFlags::SENSOR) {
                    let (sensor, target) = sensor_pair(a, b, &sensors);
                    sensor_events.write(PhysicsSensorTriggered { sensor, target });
                } else {
                    started_events.write(PhysicsCollisionStarted { a, b });
                }
            }
            RapierCollisionEvent::Stopped(a, b, flags) => {
                if !flags.contains(CollisionEventFlags::SENSOR) {
                    ended_events.write(PhysicsCollisionEnded { a, b });
                }
            }
        }
    }
}

pub fn forward_contact_force_events(
    mut rapier_events: MessageReader<RapierContactForceEvent>,
    mut force_events: MessageWriter<PhysicsContactForce2d>,
) {
    for event in rapier_events.read() {
        force_events.write(PhysicsContactForce2d {
            a: event.collider1,
            b: event.collider2,
            total_force: event.total_force,
            total_force_magnitude: event.total_force_magnitude,
            max_force_direction: event.max_force_direction,
            max_force_magnitude: event.max_force_magnitude,
        });
    }
}

fn sensor_pair(
    a: Entity,
    b: Entity,
    sensors: &Query<(), With<PhysicsSensorMarker>>,
) -> (Entity, Entity) {
    if sensors.get(a).is_ok() {
        (a, b)
    } else if sensors.get(b).is_ok() {
        (b, a)
    } else {
        (a, b)
    }
}
