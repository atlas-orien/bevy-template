use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Loading,
    MainMenu,
    Playing,
    Paused,
}

pub struct FlowPlugin;

impl Plugin for FlowPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_systems(OnEnter(AppState::Loading), enter_loading)
            .add_systems(Update, start_game.run_if(in_state(AppState::MainMenu)))
            .add_systems(Update, pause_game.run_if(in_state(AppState::Playing)))
            .add_systems(Update, resume_game.run_if(in_state(AppState::Paused)));
    }
}

fn enter_loading(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::Playing);
}

fn start_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(AppState::Playing);
    }
}

fn pause_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::Paused);
    }
}

fn resume_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::Playing);
    }
}
