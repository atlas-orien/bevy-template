pub mod demo_background;
pub mod example;

use bevy::prelude::*;

pub use demo_background::{
    DemoBackgroundLayer2d, DemoBackgroundLayer2dBundle, DemoParallaxBackgroundLayer2d,
    demo_parallax_background_system,
};

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, demo_parallax_background_system);
    }
}
