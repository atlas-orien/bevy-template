use bevy::prelude::*;
use interaction::{
    InteractionAction, InteractionEventKind, InteractionEventMessage, UiNavigationInputKind,
    UiNavigationInputMessage,
};
use prefab::ui::{DEMO_BACK_ACTION, DEMO_OPTIONS_ACTION, DEMO_QUIT_ACTION, DEMO_START_ACTION};
use render_2d::ui::{DemoMenuButtonIndex, DemoMenuFocused};

pub const DEMO_MENU_BUTTON_COUNT: usize = 4;

pub type DemoMenuButtonQuery<'world, 'state> = Query<
    'world,
    'state,
    (
        &'static DemoMenuButtonIndex,
        &'static InteractionAction,
        &'static mut DemoMenuFocused,
    ),
>;

pub fn handle_demo_ui_interactions_system(
    mut interactions: MessageReader<InteractionEventMessage>,
) {
    for interaction in interactions.read() {
        if interaction.kind != InteractionEventKind::Pressed {
            continue;
        }

        run_demo_menu_action(&interaction.action);
    }
}

pub fn handle_demo_ui_navigation_system(
    mut navigation_inputs: MessageReader<UiNavigationInputMessage>,
    mut buttons: DemoMenuButtonQuery,
) {
    for navigation_input in navigation_inputs.read() {
        let focused_index = focused_demo_menu_index(&buttons);

        match navigation_input.kind {
            UiNavigationInputKind::Previous => {
                set_demo_menu_focus(previous_demo_menu_index(focused_index), &mut buttons);
            }
            UiNavigationInputKind::Next => {
                set_demo_menu_focus(next_demo_menu_index(focused_index), &mut buttons);
            }
            UiNavigationInputKind::Activate => {
                if let Some(action) = focused_demo_menu_action(focused_index, &buttons) {
                    run_demo_menu_action(&action);
                }
            }
        }
    }
}

fn focused_demo_menu_index(buttons: &DemoMenuButtonQuery) -> usize {
    buttons
        .iter()
        .find_map(|(index, _, focus)| focus.focused.then_some(index.0))
        .unwrap_or(0)
}

fn focused_demo_menu_action(
    focused_index: usize,
    buttons: &DemoMenuButtonQuery,
) -> Option<InteractionAction> {
    buttons
        .iter()
        .find_map(|(index, action, _)| (index.0 == focused_index).then(|| action.clone()))
}

fn set_demo_menu_focus(focused_index: usize, buttons: &mut DemoMenuButtonQuery) {
    for (index, _, mut focus) in buttons.iter_mut() {
        focus.focused = index.0 == focused_index;
    }
}

fn previous_demo_menu_index(current: usize) -> usize {
    if current == 0 {
        DEMO_MENU_BUTTON_COUNT - 1
    } else {
        current - 1
    }
}

fn next_demo_menu_index(current: usize) -> usize {
    (current + 1) % DEMO_MENU_BUTTON_COUNT
}

fn run_demo_menu_action(action: &InteractionAction) {
    match action.id.as_str() {
        DEMO_START_ACTION => {
            info!("Demo UI start clicked: gameplay would start or resume the game.");
        }
        DEMO_OPTIONS_ACTION => {
            info!("Demo UI options clicked: gameplay would open the options flow.");
        }
        DEMO_QUIT_ACTION => {
            info!("Demo UI quit clicked: gameplay would request a quit or return flow.");
        }
        DEMO_BACK_ACTION => {
            info!("Demo UI back clicked: gameplay would return to the previous screen.");
        }
        _ => {}
    }
}
