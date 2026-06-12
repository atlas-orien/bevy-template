use bevy::prelude::*;

use crate::{LocalInputAction, LocalInputContext};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum KeyboardTrigger {
    JustPressed,
    Pressed,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct KeyboardBinding {
    pub context: LocalInputContext,
    pub key: KeyCode,
    pub trigger: KeyboardTrigger,
    pub action: LocalInputAction,
}

impl KeyboardBinding {
    pub const fn new(
        context: LocalInputContext,
        key: KeyCode,
        trigger: KeyboardTrigger,
        action: LocalInputAction,
    ) -> Self {
        Self {
            context,
            key,
            trigger,
            action,
        }
    }

    pub fn matches(self, keys: &ButtonInput<KeyCode>, context: LocalInputContext) -> bool {
        self.context == context
            && match self.trigger {
                KeyboardTrigger::JustPressed => keys.just_pressed(self.key),
                KeyboardTrigger::Pressed => keys.pressed(self.key),
            }
    }
}

pub const DEFAULT_KEYBOARD_BINDINGS: &[KeyboardBinding] = &[
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

pub fn collect_keyboard_actions(
    keys: &ButtonInput<KeyCode>,
    context: LocalInputContext,
    bindings: &[KeyboardBinding],
) -> Vec<LocalInputAction> {
    bindings
        .iter()
        .copied()
        .filter(|binding| binding.matches(keys, context))
        .map(|binding| binding.action)
        .collect()
}
