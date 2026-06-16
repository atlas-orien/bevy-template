use bevy::prelude::*;

use super::demo::DemoSkeletalAnimationPlugin;

pub struct SkeletalAnimation2dPlugin;

impl Plugin for SkeletalAnimation2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DemoSkeletalAnimationPlugin);
    }
}
