use bevy::prelude::*;

pub const UI_CAMERA_ORDER: isize = 100;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct UiCamera;

pub fn ui_camera_bundle() -> impl Bundle {
    (
        Camera2d,
        Camera {
            order: UI_CAMERA_ORDER,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        IsDefaultUiCamera,
        UiCamera,
    )
}
