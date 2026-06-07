use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin};
use controller::ControllerPlugin;
use ecs::EcsPlugin;
use error::ErrorPlugin;
use render_2d::Render2dPlugin;
use simulation::SimulationPlugin;

pub use error::Result;

pub fn run() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Template".to_string(),
                        resolution: (1280, 720).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins((
            ErrorPlugin,
            EcsPlugin,
            SimulationPlugin,
            ControllerPlugin,
            Render2dPlugin,
        ))
        .run();
}
