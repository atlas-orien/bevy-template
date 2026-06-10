use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhysicsShapeCastHit2d {
    pub entity: Entity,
    pub time_of_impact: f32,
    pub details: Option<PhysicsShapeCastHitDetails2d>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhysicsShapeCastHitDetails2d {
    pub witness1: Vec2,
    pub witness2: Vec2,
    pub normal1: Vec2,
    pub normal2: Vec2,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhysicsShapeCastHit3d {
    pub entity: Entity,
    pub time_of_impact: f32,
    pub details: Option<PhysicsShapeCastHitDetails3d>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhysicsShapeCastHitDetails3d {
    pub witness1: Vec3,
    pub witness2: Vec3,
    pub normal1: Vec3,
    pub normal2: Vec3,
}
