pub mod markers;
pub mod theme;

use bevy::prelude::*;

pub use markers::{HudRoot2d, MenuRoot2d};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, _app: &mut App) {}
}
