use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin};
use ecs::EcsPlugin;
use error::ErrorPlugin;
use gameplay::GameplayPlugin;
use render_2d::Render2dPlugin;

pub use error::Result;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Template".to_string(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((ErrorPlugin, EcsPlugin, GameplayPlugin, Render2dPlugin))
        .run();
}
