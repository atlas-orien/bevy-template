use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum PhysicsCollider {
    Circle { radius: f32 },
    Rectangle { width: f32, height: f32 },
    Sphere { radius: f32 },
    Cuboid { width: f32, height: f32, depth: f32 },
}

impl PhysicsCollider {
    pub const fn is_2d(self) -> bool {
        matches!(self, Self::Circle { .. } | Self::Rectangle { .. })
    }

    pub const fn is_3d(self) -> bool {
        matches!(self, Self::Sphere { .. } | Self::Cuboid { .. })
    }
}
