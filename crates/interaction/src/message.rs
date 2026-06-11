use bevy::prelude::*;

use crate::InteractionAction;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum InteractionEventKind {
    Pressed,
    Hovered,
    None,
}

#[derive(Message, Debug, Clone, Eq, PartialEq)]
pub struct InteractionEventMessage {
    pub entity: Entity,
    pub action: InteractionAction,
    pub kind: InteractionEventKind,
}
