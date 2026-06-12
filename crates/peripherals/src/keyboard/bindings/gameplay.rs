use bevy::prelude::*;

use crate::{LocalInputAction, LocalInputContext};

use super::{KeyboardBinding, KeyboardTrigger};

pub const GAMEPLAY_KEYBOARD_BINDINGS: &[KeyboardBinding] = &[
    KeyboardBinding::new(
        LocalInputContext::Gameplay,
        KeyCode::KeyW,
        KeyboardTrigger::Pressed,
        LocalInputAction::MoveUp,
    ),
    KeyboardBinding::new(
        LocalInputContext::Gameplay,
        KeyCode::KeyS,
        KeyboardTrigger::Pressed,
        LocalInputAction::MoveDown,
    ),
    KeyboardBinding::new(
        LocalInputContext::Gameplay,
        KeyCode::KeyA,
        KeyboardTrigger::Pressed,
        LocalInputAction::MoveLeft,
    ),
    KeyboardBinding::new(
        LocalInputContext::Gameplay,
        KeyCode::KeyD,
        KeyboardTrigger::Pressed,
        LocalInputAction::MoveRight,
    ),
    KeyboardBinding::new(
        LocalInputContext::Gameplay,
        KeyCode::Space,
        KeyboardTrigger::JustPressed,
        LocalInputAction::Interact,
    ),
    KeyboardBinding::new(
        LocalInputContext::Gameplay,
        KeyCode::Escape,
        KeyboardTrigger::JustPressed,
        LocalInputAction::Pause,
    ),
];
