use bevy::prelude::*;

#[derive(Message, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct PhysicsCollisionStarted {
    pub a: Entity,
    pub b: Entity,
}

#[derive(Message, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct PhysicsCollisionEnded {
    pub a: Entity,
    pub b: Entity,
}

#[derive(Message, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct PhysicsSensorTriggered {
    pub sensor: Entity,
    pub target: Entity,
}
