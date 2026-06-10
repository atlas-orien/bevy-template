use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct PhysicsColliderDisabled;

#[derive(Component, Debug, Clone, Copy, Default, PartialEq)]
pub struct PhysicsContactSkin(pub f32);

#[derive(Component, Debug, Clone, Copy, Default, PartialEq)]
pub struct PhysicsContactForceEventThreshold(pub f32);
