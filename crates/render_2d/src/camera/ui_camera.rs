use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;

pub const UI_CAMERA_ORDER: isize = 100;

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
    pub camera_2d: Camera2d,
    pub camera: Camera,
    pub render_layers: RenderLayers,
    pub default_ui_camera: IsDefaultUiCamera,
    config: UiCameraConfig,
}

impl Default for UiCamera {
    fn default() -> Self {
        let config = UiCameraConfig::default();

        Self {
            camera_2d: Camera2d,
            camera: Camera {
                order: config.order,
                clear_color: ClearColorConfig::None,
                ..default()
            },
            render_layers: RenderLayers::layer(1),
            default_ui_camera: IsDefaultUiCamera,
            config,
        }
    }
}
