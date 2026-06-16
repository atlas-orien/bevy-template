use bevy::prelude::*;

use super::systems::FrameAnimationSystemsPlugin;

pub struct FrameAnimation2dPlugin;

impl Plugin for FrameAnimation2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameAnimationSystemsPlugin);
    }
}
