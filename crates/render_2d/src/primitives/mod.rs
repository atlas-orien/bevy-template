//! 通用 2D 表现 primitive。

pub mod atlases;
pub mod camera;
pub mod frame_animation;
pub mod images;
pub mod layers;
pub mod text;
pub mod tilemap;

pub use camera::Camera2dPlugin;
pub use frame_animation::FrameAnimation2dPlugin;
pub use layers::Layers2dPlugin;

use bevy::prelude::*;

pub struct Render2dPrimitivesPlugin;

impl Plugin for Render2dPrimitivesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((Camera2dPlugin, Layers2dPlugin, FrameAnimation2dPlugin));
    }
}
