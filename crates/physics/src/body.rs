use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub enum PhysicsBody {
    #[default]
    Dynamic,
    Static,
    Kinematic,
}
