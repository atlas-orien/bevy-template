use bevy::prelude::Vec3;
use bevy_rapier3d::prelude::{
    AdditionalMassProperties as RapierMass, Collider as RapierCollider,
    ExternalForce as RapierForce, ExternalImpulse as RapierImpulse, Friction as RapierFriction,
    Restitution as RapierRestitution, RigidBody as RapierRigidBody, Velocity as RapierVelocity,
};

use crate::{
    PhysicsAngularVelocity3d, PhysicsCollider3d, PhysicsForce3d, PhysicsImpulse3d, PhysicsMass,
    PhysicsMaterial, PhysicsRigidBody, PhysicsVelocity3d,
};

pub fn rigid_body(rigid_body: PhysicsRigidBody) -> RapierRigidBody {
    match rigid_body {
        PhysicsRigidBody::Dynamic => RapierRigidBody::Dynamic,
        PhysicsRigidBody::Static => RapierRigidBody::Fixed,
        PhysicsRigidBody::Kinematic => RapierRigidBody::KinematicVelocityBased,
    }
}

pub fn collider(collider: &PhysicsCollider3d) -> Option<RapierCollider> {
    match collider {
        PhysicsCollider3d::Sphere { radius } => Some(RapierCollider::ball(*radius)),
        PhysicsCollider3d::Cuboid {
            width,
            height,
            depth,
        } => Some(RapierCollider::cuboid(
            width * 0.5,
            height * 0.5,
            depth * 0.5,
        )),
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

pub fn force(force: PhysicsForce3d) -> RapierForce {
    RapierForce {
        force: force.0,
        torque: Vec3::ZERO,
    }
}

pub fn impulse(impulse: PhysicsImpulse3d) -> RapierImpulse {
    RapierImpulse {
        impulse: impulse.0,
        torque_impulse: Vec3::ZERO,
    }
}
