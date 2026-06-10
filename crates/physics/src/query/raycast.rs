use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhysicsRayHit2d {
    pub entity: Entity,
    pub time_of_impact: f32,
    pub point: Vec2,
    pub normal: Vec2,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhysicsRayHit3d {
    pub entity: Entity,
    pub time_of_impact: f32,
    pub point: Vec3,
    pub normal: Vec3,
}
