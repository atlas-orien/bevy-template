use bevy::prelude::*;
use gameplay::flow::GameplayState;

#[derive(Component)]
struct ScreenEntity;

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameplayState::MainMenu), enter_main_menu)
            .add_systems(OnEnter(GameplayState::Playing), enter_playing)
            .add_systems(OnExit(GameplayState::Playing), despawn_screen_entities);
    }
}

fn enter_main_menu() {
    info!("Main menu ready. Press Space to start.");
}

fn enter_playing(mut commands: Commands) {
    commands.spawn((Transform::default(), Visibility::default(), ScreenEntity));
    info!("Playing. Press Escape to pause.");
}

fn despawn_screen_entities(mut commands: Commands, entities: Query<Entity, With<ScreenEntity>>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}
