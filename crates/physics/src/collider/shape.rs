use bevy::prelude::*;

#[derive(Component, Debug, Clone, PartialEq)]
pub enum PhysicsCollider2d {
    Circle { radius: f32 },
    Rectangle { width: f32, height: f32 },
    Polyline { points: Vec<Vec2> },
    ConvexPolygon { points: Vec<Vec2> },
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum PhysicsCollider3d {
    Sphere { radius: f32 },
    Cuboid { width: f32, height: f32, depth: f32 },
}
