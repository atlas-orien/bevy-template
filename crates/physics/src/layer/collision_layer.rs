use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum PhysicsLayer {
    #[default]
    Default,
    Actor,
    Enemy,
    World,
    Sensor,
}
