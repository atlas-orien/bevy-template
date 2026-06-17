use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct PhysicsLockedAxes {
    pub translation_x: bool,
    pub translation_y: bool,
    pub translation_z: bool,
    pub rotation_x: bool,
    pub rotation_y: bool,
    pub rotation_z: bool,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct PhysicsGravityScale(pub f32);

impl Default for PhysicsGravityScale {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Component, Debug, Clone, Copy, Default, PartialEq)]
pub struct PhysicsDamping {
    pub linear: f32,
    pub angular: f32,
}

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct PhysicsCcd {
    pub enabled: bool,
}

#[derive(Component, Debug, Clone, Copy, Default, PartialEq)]
pub struct PhysicsSoftCcd {
    pub prediction: f32,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct PhysicsSleeping {
    pub enabled: bool,
    pub sleeping: bool,
}

impl Default for PhysicsSleeping {
    fn default() -> Self {
        Self {
            enabled: true,
            sleeping: false,
        }
    }
}

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct PhysicsRigidBodyDisabledMarker;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct PhysicsAdditionalSolverIterations(pub usize);
