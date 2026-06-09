use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, PartialEq)]
pub struct PhysicsForce2d(pub Vec2);

#[derive(Component, Debug, Clone, Copy, Default, PartialEq)]
pub struct PhysicsImpulse2d(pub Vec2);
