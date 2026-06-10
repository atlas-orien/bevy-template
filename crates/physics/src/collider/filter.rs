use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq)]
pub struct PhysicsCollisionGroups {
    pub memberships: u32,
    pub filters: u32,
}

impl Default for PhysicsCollisionGroups {
    fn default() -> Self {
        Self {
            memberships: u32::MAX,
            filters: u32::MAX,
        }
    }
}

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq)]
pub struct PhysicsSolverGroups {
    pub memberships: u32,
    pub filters: u32,
}

impl Default for PhysicsSolverGroups {
    fn default() -> Self {
        Self {
            memberships: u32::MAX,
            filters: u32::MAX,
        }
    }
}

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct PhysicsActiveEvents {
    pub collision: bool,
    pub contact_force: bool,
}

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq)]
pub struct PhysicsActiveCollisionTypes {
    pub dynamic_dynamic: bool,
    pub dynamic_kinematic: bool,
    pub dynamic_static: bool,
    pub kinematic_kinematic: bool,
    pub kinematic_static: bool,
    pub static_static: bool,
}

impl Default for PhysicsActiveCollisionTypes {
    fn default() -> Self {
        Self {
            dynamic_dynamic: true,
            dynamic_kinematic: true,
            dynamic_static: true,
            kinematic_kinematic: false,
            kinematic_static: false,
            static_static: false,
        }
    }
}
