use bevy::prelude::*;

#[derive(Message, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct DemoSensorTriggeredEvent {
    pub sensor: Entity,
    pub target: Entity,
}
