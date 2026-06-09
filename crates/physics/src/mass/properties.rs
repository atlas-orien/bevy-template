use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct PhysicsMass(pub f32);

impl Default for PhysicsMass {
    fn default() -> Self {
        Self(1.0)
    }
}
