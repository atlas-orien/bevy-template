use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub enum PhysicsRigidBody {
    #[default]
    Dynamic,
    Static,
    Kinematic,
}
