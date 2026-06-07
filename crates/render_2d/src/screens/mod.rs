use bevy::prelude::*;
use simulation::flow::AppState;

#[derive(Component)]
struct ScreenEntity;

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), enter_main_menu)
            .add_systems(OnEnter(AppState::Playing), enter_playing)
            .add_systems(OnExit(AppState::Playing), despawn_screen_entities);
    }
}

fn enter_main_menu() {
    info!("Main menu ready. Press Space to start.");
}

fn enter_playing(mut commands: Commands) {
    commands.spawn((
        Sprite::from_color(Color::srgb(0.10, 0.13, 0.16), Vec2::new(1280.0, 720.0)),
        Transform::from_xyz(0.0, 0.0, -10.0),
        ScreenEntity,
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.22, 0.38, 0.24), Vec2::new(1280.0, 80.0)),
        Transform::from_xyz(0.0, -250.0, -5.0),
        ScreenEntity,
    ));
    info!("Playing. Press Escape to pause.");
}

fn despawn_screen_entities(mut commands: Commands, entities: Query<Entity, With<ScreenEntity>>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}
