use avian2d::prelude::{
    AngularVelocity as AvianAngularVelocity, Collider as AvianCollider, Friction as AvianFriction,
    LinearVelocity as AvianLinearVelocity, Mass as AvianMass, Restitution as AvianRestitution,
    RigidBody as AvianRigidBody,
};

use crate::{
    PhysicsAngularVelocity2d, PhysicsBody, PhysicsCollider, PhysicsMass, PhysicsMaterial,
    PhysicsVelocity2d,
};

pub fn body(body: PhysicsBody) -> AvianRigidBody {
    match body {
        PhysicsBody::Dynamic => AvianRigidBody::Dynamic,
        PhysicsBody::Static => AvianRigidBody::Static,
        PhysicsBody::Kinematic => AvianRigidBody::Kinematic,
    }
}

pub fn collider(collider: PhysicsCollider) -> AvianCollider {
    match collider {
        PhysicsCollider::Circle { radius } => AvianCollider::circle(radius),
        PhysicsCollider::Rectangle { width, height } => AvianCollider::rectangle(width, height),
    }
}

pub fn material(material: PhysicsMaterial) -> (AvianFriction, AvianRestitution) {
    (
        AvianFriction::new(material.friction),
        AvianRestitution::new(material.restitution),
    )
}

pub fn mass(mass: PhysicsMass) -> AvianMass {
    AvianMass(mass.0)
}

pub fn velocity(velocity: PhysicsVelocity2d) -> AvianLinearVelocity {
    AvianLinearVelocity(velocity.0)
}

pub fn angular_velocity(velocity: PhysicsAngularVelocity2d) -> AvianAngularVelocity {
    AvianAngularVelocity(velocity.radians_per_second)
}
