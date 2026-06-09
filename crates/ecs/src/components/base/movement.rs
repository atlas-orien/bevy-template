use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Speed(pub f32);

#[derive(Component, Debug, Clone, Copy, Default, PartialEq)]
pub struct Velocity2d(pub Vec2);

#[derive(Component, Debug, Clone, Copy, Default, PartialEq)]
pub struct Velocity3d(pub Vec3);

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct MovementIntent {
    pub target: MovementTarget,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum MovementTarget {
    #[default]
    None,
    Direction(Vec2),
    Position(Vec2),
}

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub enum Facing {
    Left,
    #[default]
    Right,
}

impl Default for Speed {
    fn default() -> Self {
        Self(180.0)
    }
}

impl MovementIntent {
    pub fn is_moving(&self) -> bool {
        !matches!(self.target, MovementTarget::None)
    }
}
