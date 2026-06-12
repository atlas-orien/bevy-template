use bevy::prelude::*;

use super::frame::FrameAnimation2dPlugin;

pub struct Animation2dPlugin;

impl Plugin for Animation2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameAnimation2dPlugin);
    }
}
