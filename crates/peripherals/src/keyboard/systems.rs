use bevy::prelude::*;
use gameplay::api::{LocalInputContextMessage, LocalUserInputMessage};
use interaction::UiNavigationInputMessage;

use crate::{LocalInputAction, LocalInputContext};

use super::{DEFAULT_KEYBOARD_BINDINGS, collect_keyboard_actions, ui_navigation_kind_for_action};

pub fn emit_keyboard_ui_navigation_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut context_messages: MessageReader<LocalInputContextMessage>,
    mut input_context: Local<LocalInputContext>,
    mut navigation_inputs: MessageWriter<UiNavigationInputMessage>,
) {
    for context_message in context_messages.read() {
        *input_context = context_message.0;
    }

    if *input_context != LocalInputContext::UiNavigation {
        return;
    }

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

pub fn emit_keyboard_gameplay_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut context_messages: MessageReader<LocalInputContextMessage>,
    mut input_context: Local<LocalInputContext>,
    mut local_inputs: MessageWriter<LocalUserInputMessage>,
) {
    for context_message in context_messages.read() {
        *input_context = context_message.0;
    }

    if *input_context != LocalInputContext::Gameplay {
        return;
    }

    let actions = collect_keyboard_actions(
        &keys,
        LocalInputContext::Gameplay,
        DEFAULT_KEYBOARD_BINDINGS,
    );
    let mut direction = Vec2::ZERO;

    for action in actions {
        match action {
            LocalInputAction::MoveUp => direction.y += 1.0,
            LocalInputAction::MoveDown => direction.y -= 1.0,
            LocalInputAction::MoveLeft => direction.x -= 1.0,
            LocalInputAction::MoveRight => direction.x += 1.0,
            LocalInputAction::Pause => {
                local_inputs.write(LocalUserInputMessage::TogglePause);
            }
            _ => {}
        }
    }

    local_inputs.write(LocalUserInputMessage::Move(direction.normalize_or_zero()));
}
