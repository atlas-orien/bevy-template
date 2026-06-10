use bevy::prelude::*;

#[derive(Component, Debug, Clone, PartialEq)]
pub enum PhysicsCollider {
    Circle { radius: f32 },
    Rectangle { width: f32, height: f32 },
    Polyline2d { points: Vec<Vec2> },
    ConvexPolygon2d { points: Vec<Vec2> },
    Sphere { radius: f32 },
    Cuboid { width: f32, height: f32, depth: f32 },
}

impl PhysicsCollider {
    pub fn is_2d(&self) -> bool {
        matches!(
            self,
            Self::Circle { .. }
                | Self::Rectangle { .. }
                | Self::Polyline2d { .. }
                | Self::ConvexPolygon2d { .. }
        )
    }

    pub const fn is_3d(&self) -> bool {
        matches!(self, Self::Sphere { .. } | Self::Cuboid { .. })
    }
}
