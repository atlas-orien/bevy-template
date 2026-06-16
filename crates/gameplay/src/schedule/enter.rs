use bevy::prelude::*;
use prefab::Prefab;
use prefab::ui::DemoMenuPrefab;
use render_2d::camera::UiCamera;

use crate::api::{LocalInputContext, LocalInputContextMessage};
use crate::spawning::initial::spawn_initial_gameplay_plan_system;
use crate::state::{AppState, PauseState};

pub fn register_enter_schedules(app: &mut App) {
    app.add_systems(OnEnter(AppState::MainMenu), enter_main_menu)
        .add_systems(
            OnEnter(AppState::Playing),
            (
                set_gameplay_input_context,
                spawn_initial_gameplay_plan_system,
            )
                .chain(),
        )
        .add_systems(OnEnter(PauseState::Running), set_gameplay_input_context)
        .add_systems(OnEnter(PauseState::Paused), enter_paused)
        .add_systems(OnEnter(AppState::GameOver), enter_game_over);
}

fn enter_main_menu(
    mut commands: Commands,
    mut input_context: MessageWriter<LocalInputContextMessage>,
) {
    input_context.write(LocalInputContextMessage(LocalInputContext::UiNavigation));

    let ui_camera = commands.spawn(UiCamera::default()).id();
    let menu = DemoMenuPrefab.spawn(&mut commands);
    commands.entity(menu).insert(UiTargetCamera(ui_camera));

    info!("Main menu ready.");
}

fn set_gameplay_input_context(mut input_context: MessageWriter<LocalInputContextMessage>) {
    input_context.write(LocalInputContextMessage(LocalInputContext::Gameplay));
}

fn enter_paused(
    mut commands: Commands,
    mut input_context: MessageWriter<LocalInputContextMessage>,
    ui_cameras: Query<Entity, With<IsDefaultUiCamera>>,
) {
    input_context.write(LocalInputContextMessage(LocalInputContext::UiNavigation));

    let ui_camera = ui_cameras
        .iter()
        .next()
        .unwrap_or_else(|| commands.spawn(UiCamera::default()).id());
    let menu = DemoMenuPrefab.spawn(&mut commands);
    commands.entity(menu).insert(UiTargetCamera(ui_camera));

    info!("Demo paused.");
}

fn enter_game_over(
    mut commands: Commands,
    mut input_context: MessageWriter<LocalInputContextMessage>,
    ui_cameras: Query<Entity, With<IsDefaultUiCamera>>,
) {
    input_context.write(LocalInputContextMessage(LocalInputContext::UiNavigation));

    let ui_camera = ui_cameras
        .iter()
        .next()
        .unwrap_or_else(|| commands.spawn(UiCamera::default()).id());
    let menu = DemoMenuPrefab.spawn(&mut commands);
    commands.entity(menu).insert(UiTargetCamera(ui_camera));

    info!("Demo game over.");
}
