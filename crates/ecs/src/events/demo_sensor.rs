//! Demo 感应区触发事件。

use bevy::prelude::*;

#[derive(Message, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct DemoSensorTriggeredEvent {
    pub sensor: Entity,
    pub target: Entity,
}
