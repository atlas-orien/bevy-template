use bevy::app::AppExit;
use bevy::prelude::*;
use interaction::{
    InteractionAction, InteractionEventKind, InteractionEventMessage, UiNavigationInputKind,
    UiNavigationInputMessage,
};
use prefab::ui::{DEMO_BACK_ACTION, DEMO_OPTIONS_ACTION, DEMO_QUIT_ACTION, DEMO_START_ACTION};
use render_2d::ui::{DemoMenuButtonIndex, DemoMenuFocused};

use crate::state::AppState;

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
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut app_exit: MessageWriter<AppExit>,
) {
    for interaction in interactions.read() {
        if interaction.kind != InteractionEventKind::Pressed {
            continue;
        }

        run_demo_menu_action(&interaction.action, &state, &mut next_state, &mut app_exit);
    }
}

pub fn handle_demo_ui_navigation_system(
    mut navigation_inputs: MessageReader<UiNavigationInputMessage>,
    mut buttons: DemoMenuButtonQuery,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut app_exit: MessageWriter<AppExit>,
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
                    run_demo_menu_action(&action, &state, &mut next_state, &mut app_exit);
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

fn run_demo_menu_action(
    action: &InteractionAction,
    state: &State<AppState>,
    next_state: &mut NextState<AppState>,
    app_exit: &mut MessageWriter<AppExit>,
) {
    match action.id.as_str() {
        DEMO_START_ACTION => {
            next_state.set(AppState::Playing);
        }
        DEMO_OPTIONS_ACTION => {
            info!("Demo UI options clicked: gameplay would open the options flow.");
        }
        DEMO_QUIT_ACTION => {
            app_exit.write(AppExit::Success);
        }
        DEMO_BACK_ACTION => {
            if state.get() == &AppState::Paused {
                next_state.set(AppState::Playing);
            } else {
                info!("Demo UI back clicked: no previous screen is active.");
            }
        }
        _ => {}
    }
}
