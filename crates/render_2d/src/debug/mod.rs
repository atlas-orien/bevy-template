pub mod example;

use bevy::prelude::*;

pub struct DebugRenderPlugin;

impl Plugin for DebugRenderPlugin {
    fn build(&self, _app: &mut App) {}
}
