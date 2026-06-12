//! Keyboard adapters for Bevy App local input.

use bevy::prelude::*;
use interaction::{UiNavigationInputKind, UiNavigationInputMessage};

pub fn emit_keyboard_ui_navigation_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut navigation_inputs: MessageWriter<UiNavigationInputMessage>,
) {
    if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::ArrowLeft) {
        navigation_inputs.write(UiNavigationInputMessage {
            kind: UiNavigationInputKind::Previous,
        });
    }

    if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::ArrowRight) {
        navigation_inputs.write(UiNavigationInputMessage {
            kind: UiNavigationInputKind::Next,
        });
    }

    if keys.just_pressed(KeyCode::Enter) {
        navigation_inputs.write(UiNavigationInputMessage {
            kind: UiNavigationInputKind::Activate,
        });
    }
}
