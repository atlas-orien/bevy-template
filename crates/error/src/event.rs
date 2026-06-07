use bevy::prelude::*;

use crate::GameError;

#[derive(Message, Debug)]
pub struct ErrorEvent {
    pub error: GameError,
}
