//! 复杂 2D 动画能力。

pub mod skeletal;

pub use skeletal::SkeletalAnimation2dPlugin;

use bevy::prelude::*;

pub struct CapabilityAnimation2dPlugin;

impl Plugin for CapabilityAnimation2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SkeletalAnimation2dPlugin);
    }
}
