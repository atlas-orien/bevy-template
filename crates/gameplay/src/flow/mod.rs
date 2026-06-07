use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameplayState {
    #[default]
    Loading,
    MainMenu,
    Playing,
    Paused,
}

pub struct FlowPlugin;

impl Plugin for FlowPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameplayState>()
            .add_systems(OnEnter(GameplayState::Loading), enter_loading)
            .add_systems(Update, start_game.run_if(in_state(GameplayState::MainMenu)))
            .add_systems(Update, pause_game.run_if(in_state(GameplayState::Playing)))
            .add_systems(Update, resume_game.run_if(in_state(GameplayState::Paused)));
    }
}

fn enter_loading(mut next_state: ResMut<NextState<GameplayState>>) {
    next_state.set(GameplayState::MainMenu);
}

fn start_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameplayState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(GameplayState::Playing);
    }
}

fn pause_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameplayState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(GameplayState::Paused);
    }
}

fn resume_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameplayState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(GameplayState::Playing);
    }
}
