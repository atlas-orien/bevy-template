use bevy::prelude::*;

use crate::{LocalInputAction, LocalInputContext};

use super::{KeyboardBinding, KeyboardTrigger};

pub const UI_NAVIGATION_KEYBOARD_BINDINGS: &[KeyboardBinding] = &[
    KeyboardBinding::new(
        LocalInputContext::UiNavigation,
        KeyCode::ArrowUp,
        KeyboardTrigger::JustPressed,
        LocalInputAction::UiPrevious,
    ),
    KeyboardBinding::new(
        LocalInputContext::UiNavigation,
        KeyCode::ArrowLeft,
        KeyboardTrigger::JustPressed,
        LocalInputAction::UiPrevious,
    ),
    KeyboardBinding::new(
        LocalInputContext::UiNavigation,
        KeyCode::ArrowDown,
        KeyboardTrigger::JustPressed,
        LocalInputAction::UiNext,
    ),
    KeyboardBinding::new(
        LocalInputContext::UiNavigation,
        KeyCode::ArrowRight,
        KeyboardTrigger::JustPressed,
        LocalInputAction::UiNext,
    ),
    KeyboardBinding::new(
        LocalInputContext::UiNavigation,
        KeyCode::Enter,
        KeyboardTrigger::JustPressed,
        LocalInputAction::UiActivate,
    ),
];
