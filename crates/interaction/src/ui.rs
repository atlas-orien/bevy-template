use bevy::prelude::*;

use crate::{InteractionAction, InteractionEventKind, InteractionEventMessage};

type UiButtonInteractionQuery<'world, 'state> = Query<
    'world,
    'state,
    (Entity, &'static Interaction, &'static InteractionAction),
    (Changed<Interaction>, With<Button>),
>;

pub fn emit_ui_button_interactions_system(
    buttons: UiButtonInteractionQuery,
    mut interactions: MessageWriter<InteractionEventMessage>,
) {
    for (entity, interaction, action) in &buttons {
        interactions.write(InteractionEventMessage {
            entity,
            action: action.clone(),
            kind: match *interaction {
                Interaction::Pressed => InteractionEventKind::Pressed,
                Interaction::Hovered => InteractionEventKind::Hovered,
                Interaction::None => InteractionEventKind::None,
            },
        });
    }
}
