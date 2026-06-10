use bevy::prelude::Vec2;
use bevy_rapier2d::prelude::{
    AdditionalMassProperties as RapierMass, Collider as RapierCollider, Friction as RapierFriction,
    Restitution as RapierRestitution, RigidBody as RapierRigidBody, Velocity as RapierVelocity,
};

use crate::{
    PhysicsAngularVelocity2d, PhysicsCollider, PhysicsMass, PhysicsMaterial, PhysicsRigidBody,
    PhysicsVelocity2d,
};

pub fn rigid_body(rigid_body: PhysicsRigidBody) -> RapierRigidBody {
    match rigid_body {
        PhysicsRigidBody::Dynamic => RapierRigidBody::Dynamic,
        PhysicsRigidBody::Static => RapierRigidBody::Fixed,
        PhysicsRigidBody::Kinematic => RapierRigidBody::KinematicVelocityBased,
    }
}

pub fn collider(collider: PhysicsCollider) -> RapierCollider {
    match collider {
        PhysicsCollider::Circle { radius } => RapierCollider::ball(radius),
        PhysicsCollider::Rectangle { width, height } => {
            RapierCollider::cuboid(width * 0.5, height * 0.5)
        }
        PhysicsCollider::Sphere { .. } | PhysicsCollider::Cuboid { .. } => {
            unreachable!("3D colliders are handled by the Rapier 3D adapter")
        }
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
