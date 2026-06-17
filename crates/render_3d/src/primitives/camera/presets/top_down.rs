//! 俯视 3D 场景相机。

use bevy::prelude::*;

use super::FixedCamera3dBundle;

const TOP_DOWN_CAMERA_3D_TRANSLATION: Vec3 = Vec3::new(0.0, 10.0, 0.0);
const TOP_DOWN_CAMERA_3D_TARGET: Vec3 = Vec3::ZERO;
const TOP_DOWN_CAMERA_3D_UP: Vec3 = Vec3::Z;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct TopDownCamera3dMarker;

#[derive(Bundle)]
pub struct TopDownCamera3dBundle {
    fixed_camera: FixedCamera3dBundle,
    marker: TopDownCamera3dMarker,
}

impl Default for TopDownCamera3dBundle {
    fn default() -> Self {
        Self {
            fixed_camera: FixedCamera3dBundle::new(
                TOP_DOWN_CAMERA_3D_TRANSLATION,
                TOP_DOWN_CAMERA_3D_TARGET,
                TOP_DOWN_CAMERA_3D_UP,
                default(),
            ),
            marker: TopDownCamera3dMarker,
        }
    }
}
