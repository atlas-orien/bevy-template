//! Keyboard adapters for Bevy App local input.

mod bindings;
mod routing;

use bevy::prelude::*;
use interaction::UiNavigationInputMessage;

use crate::LocalInputContext;

pub use bindings::{
    DEFAULT_KEYBOARD_BINDINGS, KeyboardBinding, KeyboardTrigger, collect_keyboard_actions,
};
pub use routing::ui_navigation_kind_for_action;

pub fn emit_keyboard_ui_navigation_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut navigation_inputs: MessageWriter<UiNavigationInputMessage>,
) {
    for action in collect_keyboard_actions(
        &keys,
        LocalInputContext::UiNavigation,
        DEFAULT_KEYBOARD_BINDINGS,
    ) {
        let Some(kind) = ui_navigation_kind_for_action(action) else {
            continue;
        };

        navigation_inputs.write(UiNavigationInputMessage { kind });
    }
}
