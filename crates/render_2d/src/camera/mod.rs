use bevy::prelude::*;
use gameplay::flow::GameplayState;

#[derive(Component)]
struct MainCamera;

pub struct Camera2dPlugin;

impl Plugin for Camera2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameplayState::Playing), spawn_camera)
            .add_systems(OnExit(GameplayState::Playing), despawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, MainCamera));
}

fn despawn_camera(mut commands: Commands, cameras: Query<Entity, With<MainCamera>>) {
    for entity in &cameras {
        commands.entity(entity).despawn();
    }
}
