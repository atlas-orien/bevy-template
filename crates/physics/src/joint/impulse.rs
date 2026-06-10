use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct PhysicsImpulseJoint2d {
    pub parent: Entity,
    pub kind: PhysicsJointKind2d,
    pub contacts_enabled: bool,
}

impl PhysicsImpulseJoint2d {
    pub fn new(parent: Entity, kind: PhysicsJointKind2d) -> Self {
        Self {
            parent,
            kind,
            contacts_enabled: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PhysicsJointKind2d {
    Fixed {
        local_anchor1: Vec2,
        local_anchor2: Vec2,
    },
    Revolute {
        local_anchor1: Vec2,
        local_anchor2: Vec2,
    },
    Prismatic {
        axis: Vec2,
        local_anchor1: Vec2,
        local_anchor2: Vec2,
    },
    Rope {
        max_distance: f32,
        local_anchor1: Vec2,
        local_anchor2: Vec2,
    },
    Spring {
        rest_length: f32,
        stiffness: f32,
        damping: f32,
        local_anchor1: Vec2,
        local_anchor2: Vec2,
    },
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct PhysicsImpulseJoint3d {
    pub parent: Entity,
    pub kind: PhysicsJointKind3d,
    pub contacts_enabled: bool,
}

impl PhysicsImpulseJoint3d {
    pub fn new(parent: Entity, kind: PhysicsJointKind3d) -> Self {
        Self {
            parent,
            kind,
            contacts_enabled: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PhysicsJointKind3d {
    Fixed {
        local_anchor1: Vec3,
        local_anchor2: Vec3,
    },
    Revolute {
        axis: Vec3,
        local_anchor1: Vec3,
        local_anchor2: Vec3,
    },
    Prismatic {
        axis: Vec3,
        local_anchor1: Vec3,
        local_anchor2: Vec3,
    },
    Rope {
        max_distance: f32,
        local_anchor1: Vec3,
        local_anchor2: Vec3,
    },
    Spring {
        rest_length: f32,
        stiffness: f32,
        damping: f32,
        local_anchor1: Vec3,
        local_anchor2: Vec3,
    },
}
