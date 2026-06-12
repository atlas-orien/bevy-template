//! Keyboard adapters for Bevy App local input.

mod bindings;
mod routing;
mod systems;

pub use bindings::{
    DEFAULT_KEYBOARD_BINDINGS, KeyboardBinding, KeyboardTrigger, collect_keyboard_actions,
};
pub use routing::ui_navigation_kind_for_action;
pub use systems::{emit_keyboard_gameplay_input_system, emit_keyboard_ui_navigation_system};
