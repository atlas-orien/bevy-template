//! Demo 菜单的焦点导航与按钮动作分发。

use bevy::app::AppExit;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use interaction::{
    InteractionAction, InteractionEventKind, InteractionEventMessage, UiNavigationInputKind,
    UiNavigationInputMessage,
};
use prefab::ui::{DEMO_MENU_ITEMS, DemoMenuAction};
use render_2d::ui::{DemoMenuButtonIndex, DemoMenuFocused};

use crate::api::{RuntimeUpdateMessage, RuntimeUpdateSender};
use crate::state::{AppState, PauseState};

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
    mut actions: DemoMenuActionContext,
) {
    for interaction in interactions.read() {
        if interaction.kind != InteractionEventKind::Pressed {
            continue;
        }

        actions.run(&interaction.action);
    }
}

pub fn handle_demo_ui_navigation_system(
    mut navigation_inputs: MessageReader<UiNavigationInputMessage>,
    mut buttons: DemoMenuButtonQuery,
    mut actions: DemoMenuActionContext,
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
                    actions.run(&action);
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
        DEMO_MENU_ITEMS.len() - 1
    } else {
        current - 1
    }
}

fn next_demo_menu_index(current: usize) -> usize {
    (current + 1) % DEMO_MENU_ITEMS.len()
}

#[derive(SystemParam)]
pub struct DemoMenuActionContext<'w> {
    app_state: Res<'w, State<AppState>>,
    pause_state: Option<Res<'w, State<PauseState>>>,
    next_state: ResMut<'w, NextState<AppState>>,
    next_pause_state: Option<ResMut<'w, NextState<PauseState>>>,
    app_exit: MessageWriter<'w, AppExit>,
    update_sender: Option<Res<'w, RuntimeUpdateSender>>,
}

impl DemoMenuActionContext<'_> {
    fn run(&mut self, action: &InteractionAction) {
        match DemoMenuAction::from_id(action.id.as_str()) {
            Some(DemoMenuAction::Start) => {
                self.next_state.set(AppState::Playing);
            }
            Some(DemoMenuAction::Options) => {
                info!("Demo UI options clicked: gameplay would open the options flow.");
            }
            Some(DemoMenuAction::NetworkLogin) => {
                if let Some(update_sender) = &self.update_sender {
                    let _ =
                        update_sender.send(RuntimeUpdateMessage::demo_network_login_requested());
                }
                info!("Demo UI network login test requested.");
            }
            Some(DemoMenuAction::Quit) => {
                self.app_exit.write(AppExit::Success);
            }
            Some(DemoMenuAction::Back) => {
                if self.app_state.get() == &AppState::Playing
                    && self
                        .pause_state
                        .as_deref()
                        .is_some_and(|state| state.get() == &PauseState::Paused)
                {
                    if let Some(next_pause_state) = &mut self.next_pause_state {
                        next_pause_state.set(PauseState::Running);
                    }
                } else {
                    info!("Demo UI back clicked: no previous screen is active.");
                }
            }
            None => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn previous_demo_menu_index_wraps_from_first_to_last() {
        assert_eq!(previous_demo_menu_index(0), DEMO_MENU_ITEMS.len() - 1);
    }

    #[test]
    fn previous_demo_menu_index_moves_back_from_middle() {
        assert_eq!(previous_demo_menu_index(2), 1);
    }

    #[test]
    fn next_demo_menu_index_wraps_from_last_to_first() {
        assert_eq!(next_demo_menu_index(DEMO_MENU_ITEMS.len() - 1), 0);
    }

    #[test]
    fn next_demo_menu_index_moves_forward_from_middle() {
        assert_eq!(next_demo_menu_index(1), 2);
    }

    #[test]
    fn demo_menu_action_ids_resolve_known_actions() {
        for item in DEMO_MENU_ITEMS {
            assert_eq!(DemoMenuAction::from_id(item.action.id()), Some(item.action));
        }
    }

    #[test]
    fn demo_menu_action_unknown_id_returns_none() {
        assert_eq!(DemoMenuAction::from_id("demo:missing"), None);
    }
}
