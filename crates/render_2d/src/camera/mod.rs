pub mod example;
pub mod ui_camera;

use bevy::prelude::*;

pub use ui_camera::{UI_CAMERA_ORDER, UiCamera, UiCameraConfig};

pub struct Camera2dPlugin;

impl Plugin for Camera2dPlugin {
    fn build(&self, _app: &mut App) {}
}
