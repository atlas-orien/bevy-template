use bevy::prelude::*;

use super::AppState;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_systems(OnEnter(AppState::Loading), enter_loading);
    }
}

fn enter_loading(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::MainMenu);
}
