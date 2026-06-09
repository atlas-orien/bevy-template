use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, PartialEq)]
pub struct PhysicsVelocity2d(pub Vec2);

#[derive(Component, Debug, Clone, Copy, Default, PartialEq)]
pub struct PhysicsAngularVelocity2d {
    pub radians_per_second: f32,
}
