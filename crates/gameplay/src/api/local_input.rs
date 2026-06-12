use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum LocalInputContext {
    #[default]
    UiNavigation,
    Gameplay,
    TextEntry,
}

#[derive(Message, Debug, Clone, Copy, PartialEq)]
pub enum LocalUserInputMessage {
    Move(Vec2),
    TogglePause,
}

#[derive(Message, Debug, Clone, Copy, Eq, PartialEq)]
pub struct LocalInputContextMessage(pub LocalInputContext);
