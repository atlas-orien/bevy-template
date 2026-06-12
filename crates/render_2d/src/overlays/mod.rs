pub mod demo_health_bar;
pub mod example;

use bevy::prelude::*;

pub use demo_health_bar::{
    DemoHealthBarBackground2dBundle, DemoHealthBarFill2d, DemoHealthBarFill2dBundle,
    DemoHealthBarOverlay2d, DemoHealthBarOverlay2dBundle, demo_health_bar_system,
};

pub struct OverlaysPlugin;

impl Plugin for OverlaysPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, demo_health_bar_system);
    }
}
