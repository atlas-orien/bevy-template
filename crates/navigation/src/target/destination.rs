use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, PartialEq)]
pub enum NavigationTarget2d {
    #[default]
    None,
    Position(Vec2),
}

#[derive(Component, Debug, Clone, Copy, Default, PartialEq)]
pub enum NavigationTarget3d {
    #[default]
    None,
    Position(Vec3),
}

impl NavigationTarget2d {
    pub fn position(self) -> Option<Vec2> {
        match self {
            Self::None => None,
            Self::Position(position) => Some(position),
        }
    }
}

impl NavigationTarget3d {
    pub fn position(self) -> Option<Vec3> {
        match self {
            Self::None => None,
            Self::Position(position) => Some(position),
        }
    }
}
