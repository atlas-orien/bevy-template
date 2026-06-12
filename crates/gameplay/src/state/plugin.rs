use bevy::prelude::*;

use super::{AppState, PauseState};

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_sub_state::<PauseState>()
            .add_systems(OnEnter(AppState::Loading), enter_loading);
    }
}

fn enter_loading(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::MainMenu);
}
