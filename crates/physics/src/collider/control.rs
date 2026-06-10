use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct PhysicsColliderDisabled;

#[derive(Component, Debug, Clone, Copy, Default, PartialEq)]
pub struct PhysicsContactSkin(pub f32);

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct PhysicsContactForceEventThreshold(pub f32);

impl Default for PhysicsContactForceEventThreshold {
    fn default() -> Self {
        Self(f32::MAX)
    }
}
