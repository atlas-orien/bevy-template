//! Runnable Bevy application assembly.

use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin};

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
        .run();
}
