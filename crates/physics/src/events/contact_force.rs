use bevy::prelude::*;

#[derive(Message, Debug, Clone, Copy, PartialEq)]
pub struct PhysicsContactForce2d {
    pub a: Entity,
    pub b: Entity,
    pub total_force: Vec2,
    pub total_force_magnitude: f32,
    pub max_force_direction: Vec2,
    pub max_force_magnitude: f32,
}

#[derive(Message, Debug, Clone, Copy, PartialEq)]
pub struct PhysicsContactForce3d {
    pub a: Entity,
    pub b: Entity,
    pub total_force: Vec3,
    pub total_force_magnitude: f32,
    pub max_force_direction: Vec3,
    pub max_force_magnitude: f32,
}
