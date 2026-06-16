use bevy::prelude::*;

use crate::capabilities::animation::SkeletalAnimation2dPlugin;
use crate::primitives::animation::FrameAnimation2dPlugin;

pub struct Animation2dPlugin;

impl Plugin for Animation2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((FrameAnimation2dPlugin, SkeletalAnimation2dPlugin));
    }
}
