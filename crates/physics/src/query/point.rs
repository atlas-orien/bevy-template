use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhysicsPointProjection2d {
    pub entity: Entity,
    pub point: Vec2,
    pub is_inside: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhysicsPointProjection3d {
    pub entity: Entity,
    pub point: Vec3,
    pub is_inside: bool,
}
