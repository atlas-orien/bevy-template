use bevy::prelude::*;

use crate::camera::base::{BaseCamera2dBundle, BaseCamera2dConfig};

pub const UI_CAMERA_ORDER: isize = 100;
const UI_CAMERA_LAYER: usize = 1;
const UI_CAMERA_Z: f32 = 1000.0;

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq)]
struct UiCameraConfig {
    pub order: isize,
}

impl Default for UiCameraConfig {
    fn default() -> Self {
        Self {
            order: UI_CAMERA_ORDER,
        }
    }
}

#[derive(Bundle)]
pub struct UiCamera {
    camera: BaseCamera2dBundle,
    pub default_ui_camera: IsDefaultUiCamera,
    config: UiCameraConfig,
}

impl Default for UiCamera {
    fn default() -> Self {
        let config = UiCameraConfig::default();

        Self {
            camera: BaseCamera2dBundle::new(BaseCamera2dConfig {
                order: config.order,
                layer: UI_CAMERA_LAYER,
                z: UI_CAMERA_Z,
                clear_color: ClearColorConfig::None,
            }),
            default_ui_camera: IsDefaultUiCamera,
            config,
        }
    }
}
