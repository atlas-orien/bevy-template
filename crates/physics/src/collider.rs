use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum PhysicsCollider {
    Circle { radius: f32 },
    Rectangle { width: f32, height: f32 },
}

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct PhysicsSensor;
