//! 通用 2D 动画 primitive。

pub mod frame;

pub use frame::FrameAnimation2dPlugin;

use bevy::prelude::*;

pub struct PrimitiveAnimation2dPlugin;

impl Plugin for PrimitiveAnimation2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameAnimation2dPlugin);
    }
}
