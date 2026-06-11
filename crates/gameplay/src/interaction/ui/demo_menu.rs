use bevy::prelude::*;
use interaction::{InteractionEventKind, InteractionEventMessage};
use prefab::ui::{DEMO_OPTIONS_ACTION, DEMO_QUIT_ACTION, DEMO_START_ACTION};

pub fn handle_demo_ui_interactions_system(
    mut interactions: MessageReader<InteractionEventMessage>,
) {
    for interaction in interactions.read() {
        if interaction.kind != InteractionEventKind::Pressed {
            continue;
        }

        match interaction.action.id.as_str() {
            DEMO_START_ACTION => {
                info!("Demo UI start clicked: gameplay would start or resume the game.");
            }
            DEMO_OPTIONS_ACTION => {
                info!("Demo UI options clicked: gameplay would open the options flow.");
            }
            DEMO_QUIT_ACTION => {
                info!("Demo UI quit clicked: gameplay would request a quit or return flow.");
            }
            _ => {}
        }
    }
}
