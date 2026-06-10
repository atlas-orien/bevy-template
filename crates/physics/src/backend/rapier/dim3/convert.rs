use bevy::prelude::Vec3;
use bevy_rapier3d::prelude::{
    ActiveCollisionTypes as RapierActiveCollisionTypes, ActiveEvents as RapierActiveEvents,
    AdditionalMassProperties as RapierMass, AdditionalSolverIterations as RapierSolverIterations,
    Ccd as RapierCcd, Collider as RapierCollider, CollisionGroups as RapierCollisionGroups,
    ContactForceEventThreshold as RapierContactForceEventThreshold,
    ContactSkin as RapierContactSkin, Damping as RapierDamping, ExternalForce as RapierForce,
    ExternalImpulse as RapierImpulse, Friction as RapierFriction,
    GravityScale as RapierGravityScale, Group as RapierGroup, LockedAxes as RapierLockedAxes,
    Restitution as RapierRestitution, RigidBody as RapierRigidBody,
    RigidBodyDisabled as RapierRigidBodyDisabled, Sleeping as RapierSleeping,
    SoftCcd as RapierSoftCcd, SolverGroups as RapierSolverGroups, Velocity as RapierVelocity,
};

use crate::{
    PhysicsActiveCollisionTypes, PhysicsActiveEvents, PhysicsAdditionalSolverIterations,
    PhysicsAngularVelocity3d, PhysicsCcd, PhysicsCollider3d, PhysicsCollisionGroups,
    PhysicsContactForceEventThreshold, PhysicsContactSkin, PhysicsDamping, PhysicsForce3d,
    PhysicsGravityScale, PhysicsImpulse3d, PhysicsLockedAxes, PhysicsMass, PhysicsMaterial,
    PhysicsRigidBody, PhysicsSleeping, PhysicsSoftCcd, PhysicsSolverGroups, PhysicsVelocity3d,
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

pub fn locked_axes(locked_axes: PhysicsLockedAxes) -> RapierLockedAxes {
    let mut axes = RapierLockedAxes::empty();

    if locked_axes.translation_x {
        axes |= RapierLockedAxes::TRANSLATION_LOCKED_X;
    }
    if locked_axes.translation_y {
        axes |= RapierLockedAxes::TRANSLATION_LOCKED_Y;
    }
    if locked_axes.translation_z {
        axes |= RapierLockedAxes::TRANSLATION_LOCKED_Z;
    }
    if locked_axes.rotation_x {
        axes |= RapierLockedAxes::ROTATION_LOCKED_X;
    }
    if locked_axes.rotation_y {
        axes |= RapierLockedAxes::ROTATION_LOCKED_Y;
    }
    if locked_axes.rotation_z {
        axes |= RapierLockedAxes::ROTATION_LOCKED_Z;
    }

    axes
}

pub fn gravity_scale(gravity_scale: PhysicsGravityScale) -> RapierGravityScale {
    RapierGravityScale(gravity_scale.0)
}

pub fn damping(damping: PhysicsDamping) -> RapierDamping {
    RapierDamping {
        linear_damping: damping.linear,
        angular_damping: damping.angular,
    }
}

pub fn ccd(ccd: PhysicsCcd) -> RapierCcd {
    RapierCcd {
        enabled: ccd.enabled,
    }
}

pub fn soft_ccd(soft_ccd: PhysicsSoftCcd) -> RapierSoftCcd {
    RapierSoftCcd {
        prediction: soft_ccd.prediction,
    }
}

pub fn sleeping(sleeping: PhysicsSleeping) -> RapierSleeping {
    if sleeping.enabled {
        RapierSleeping {
            sleeping: sleeping.sleeping,
            ..Default::default()
        }
    } else {
        RapierSleeping::disabled()
    }
}

pub fn solver_iterations(iterations: PhysicsAdditionalSolverIterations) -> RapierSolverIterations {
    RapierSolverIterations(iterations.0)
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

pub fn collision_groups(groups: PhysicsCollisionGroups) -> Option<RapierCollisionGroups> {
    Some(RapierCollisionGroups::new(
        RapierGroup::from_bits(groups.memberships)?,
        RapierGroup::from_bits(groups.filters)?,
    ))
}

pub fn solver_groups(groups: PhysicsSolverGroups) -> Option<RapierSolverGroups> {
    Some(RapierSolverGroups::new(
        RapierGroup::from_bits(groups.memberships)?,
        RapierGroup::from_bits(groups.filters)?,
    ))
}

pub fn active_events(events: PhysicsActiveEvents) -> RapierActiveEvents {
    let mut active_events = RapierActiveEvents::empty();

    if events.collision {
        active_events |= RapierActiveEvents::COLLISION_EVENTS;
    }
    if events.contact_force {
        active_events |= RapierActiveEvents::CONTACT_FORCE_EVENTS;
    }

    active_events
}

pub fn active_collision_types(types: PhysicsActiveCollisionTypes) -> RapierActiveCollisionTypes {
    let mut active_types = RapierActiveCollisionTypes::empty();

    if types.dynamic_dynamic {
        active_types |= RapierActiveCollisionTypes::DYNAMIC_DYNAMIC;
    }
    if types.dynamic_kinematic {
        active_types |= RapierActiveCollisionTypes::DYNAMIC_KINEMATIC;
    }
    if types.dynamic_static {
        active_types |= RapierActiveCollisionTypes::DYNAMIC_STATIC;
    }
    if types.kinematic_kinematic {
        active_types |= RapierActiveCollisionTypes::KINEMATIC_KINEMATIC;
    }
    if types.kinematic_static {
        active_types |= RapierActiveCollisionTypes::KINEMATIC_STATIC;
    }
    if types.static_static {
        active_types |= RapierActiveCollisionTypes::STATIC_STATIC;
    }

    active_types
}

pub fn contact_skin(contact_skin: PhysicsContactSkin) -> RapierContactSkin {
    RapierContactSkin(contact_skin.0)
}

pub fn contact_force_threshold(
    threshold: PhysicsContactForceEventThreshold,
) -> RapierContactForceEventThreshold {
    RapierContactForceEventThreshold(threshold.0)
}

pub fn rigid_body_disabled() -> RapierRigidBodyDisabled {
    RapierRigidBodyDisabled
}
