use bevy::prelude::Vec2;
use bevy_rapier2d::prelude::{
    AdditionalMassProperties as RapierMass, Collider as RapierCollider,
    ExternalForce as RapierForce, ExternalImpulse as RapierImpulse, Friction as RapierFriction,
    Restitution as RapierRestitution, RigidBody as RapierRigidBody, Velocity as RapierVelocity,
};

use crate::{
    PhysicsAngularVelocity2d, PhysicsCollider2d, PhysicsForce2d, PhysicsImpulse2d, PhysicsMass,
    PhysicsMaterial, PhysicsRigidBody, PhysicsVelocity2d,
};

pub fn rigid_body(rigid_body: PhysicsRigidBody) -> RapierRigidBody {
    match rigid_body {
        PhysicsRigidBody::Dynamic => RapierRigidBody::Dynamic,
        PhysicsRigidBody::Static => RapierRigidBody::Fixed,
        PhysicsRigidBody::Kinematic => RapierRigidBody::KinematicVelocityBased,
    }
}

pub fn collider(collider: &PhysicsCollider2d) -> Option<RapierCollider> {
    match collider {
        PhysicsCollider2d::Circle { radius } => Some(RapierCollider::ball(*radius)),
        PhysicsCollider2d::Rectangle { width, height } => {
            Some(RapierCollider::cuboid(width * 0.5, height * 0.5))
        }
        PhysicsCollider2d::Polyline { points } => {
            Some(RapierCollider::polyline(points.clone(), None))
        }
        PhysicsCollider2d::ConvexPolygon { points } => RapierCollider::convex_hull(points),
    }
}

pub fn material(material: PhysicsMaterial) -> (RapierFriction, RapierRestitution) {
    (
        RapierFriction::new(material.friction),
        RapierRestitution::new(material.restitution),
    )
}

pub fn mass(mass: PhysicsMass) -> RapierMass {
    RapierMass::Mass(mass.0)
}

pub fn linear_velocity(velocity: PhysicsVelocity2d) -> RapierVelocity {
    RapierVelocity::linear(velocity.0)
}

pub fn angular_velocity(velocity: PhysicsAngularVelocity2d) -> RapierVelocity {
    RapierVelocity {
        linear: Vec2::ZERO,
        angular: velocity.radians_per_second,
    }
}

pub fn force(force: PhysicsForce2d) -> RapierForce {
    RapierForce {
        force: force.0,
        torque: 0.0,
    }
}

pub fn impulse(impulse: PhysicsImpulse2d) -> RapierImpulse {
    RapierImpulse {
        impulse: impulse.0,
        torque_impulse: 0.0,
    }
}
