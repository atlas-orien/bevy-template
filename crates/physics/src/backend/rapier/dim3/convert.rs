use bevy::prelude::Vec3;
use bevy_rapier3d::prelude::{
    AdditionalMassProperties as RapierMass, Collider as RapierCollider, Friction as RapierFriction,
    Restitution as RapierRestitution, RigidBody as RapierRigidBody, Velocity as RapierVelocity,
};

use crate::{
    PhysicsAngularVelocity3d, PhysicsBody, PhysicsCollider, PhysicsMass, PhysicsMaterial,
    PhysicsVelocity3d,
};

pub fn body(body: PhysicsBody) -> RapierRigidBody {
    match body {
        PhysicsBody::Dynamic => RapierRigidBody::Dynamic,
        PhysicsBody::Static => RapierRigidBody::Fixed,
        PhysicsBody::Kinematic => RapierRigidBody::KinematicVelocityBased,
    }
}

pub fn collider(collider: PhysicsCollider) -> RapierCollider {
    match collider {
        PhysicsCollider::Sphere { radius } => RapierCollider::ball(radius),
        PhysicsCollider::Cuboid {
            width,
            height,
            depth,
        } => RapierCollider::cuboid(width * 0.5, height * 0.5, depth * 0.5),
        PhysicsCollider::Circle { .. } | PhysicsCollider::Rectangle { .. } => {
            unreachable!("2D colliders are handled by the Rapier 2D adapter")
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

pub fn linear_velocity(velocity: PhysicsVelocity3d) -> RapierVelocity {
    RapierVelocity::linear(velocity.0)
}

pub fn angular_velocity(velocity: PhysicsAngularVelocity3d) -> RapierVelocity {
    RapierVelocity {
        linear: Vec3::ZERO,
        angular: velocity.0,
    }
}
