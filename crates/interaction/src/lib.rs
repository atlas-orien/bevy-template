pub mod action;
pub mod message;
pub mod ui;

pub use action::InteractionAction;
pub use error::Result;
pub use message::{
    InteractionEventKind, InteractionEventMessage, UiNavigationInputKind, UiNavigationInputMessage,
};

use bevy::prelude::*;

use self::ui::emit_ui_button_interactions_system;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<InteractionEventMessage>()
            .add_message::<UiNavigationInputMessage>()
            .add_systems(Update, emit_ui_button_interactions_system);
    }
}
