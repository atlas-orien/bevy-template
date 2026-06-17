//! 固定不动的 3D 场景相机。

use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;

use crate::primitives::camera::base::{BaseCamera3dBundle, BaseCamera3dConfig};
use crate::primitives::transforms::LookAtTransform3d;

const FIXED_CAMERA_3D_ORDER: isize = 0;
const FIXED_CAMERA_3D_LAYER: usize = 0;
const FIXED_CAMERA_3D_TRANSLATION: Vec3 = Vec3::new(0.0, 3.2, 7.0);
const FIXED_CAMERA_3D_TARGET: Vec3 = Vec3::new(0.0, 0.7, 0.0);
const FIXED_CAMERA_3D_UP: Vec3 = Vec3::Y;
const FIXED_CAMERA_3D_CLEAR_COLOR: ClearColorConfig =
    ClearColorConfig::Custom(Color::srgb(0.08, 0.09, 0.11));

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct FixedCamera3dMarker;

#[derive(Bundle)]
pub struct FixedCamera3dBundle {
    camera: BaseCamera3dBundle,
    marker: FixedCamera3dMarker,
}

impl Default for FixedCamera3dBundle {
    fn default() -> Self {
        Self::new(
            FIXED_CAMERA_3D_TRANSLATION,
            FIXED_CAMERA_3D_TARGET,
            FIXED_CAMERA_3D_UP,
            FIXED_CAMERA_3D_CLEAR_COLOR,
        )
    }
}

impl FixedCamera3dBundle {
    pub fn new(translation: Vec3, target: Vec3, up: Vec3, clear_color: ClearColorConfig) -> Self {
        Self {
            camera: BaseCamera3dBundle::new(BaseCamera3dConfig {
                camera: Camera {
                    order: FIXED_CAMERA_3D_ORDER,
                    clear_color,
                    ..default()
                },
                render_layers: RenderLayers::layer(FIXED_CAMERA_3D_LAYER),
                transform: LookAtTransform3d::new(translation, target, up).into(),
                ..default()
            }),
            marker: FixedCamera3dMarker,
        }
    }
}
