use bevy::prelude::*;

use crate::{LocalInputAction, LocalInputContext};

use super::{GAMEPLAY_KEYBOARD_BINDINGS, UI_NAVIGATION_KEYBOARD_BINDINGS};

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

pub const DEFAULT_KEYBOARD_BINDINGS: &[&[KeyboardBinding]] =
    &[UI_NAVIGATION_KEYBOARD_BINDINGS, GAMEPLAY_KEYBOARD_BINDINGS];

pub fn collect_keyboard_actions(
    keys: &ButtonInput<KeyCode>,
    context: LocalInputContext,
    binding_groups: &[&[KeyboardBinding]],
) -> Vec<LocalInputAction> {
    binding_groups
        .iter()
        .flat_map(|bindings| bindings.iter())
        .copied()
        .filter(|binding| binding.matches(keys, context))
        .map(|binding| binding.action)
        .collect()
}
