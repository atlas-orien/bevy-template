//! 等距视角 3D 场景相机。

use bevy::prelude::*;

use super::FixedCamera3dBundle;

const ISOMETRIC_CAMERA_3D_TRANSLATION: Vec3 = Vec3::new(6.0, 6.0, 6.0);
const ISOMETRIC_CAMERA_3D_TARGET: Vec3 = Vec3::ZERO;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct IsometricCamera3dMarker;

#[derive(Bundle)]
pub struct IsometricCamera3dBundle {
    fixed_camera: FixedCamera3dBundle,
    marker: IsometricCamera3dMarker,
}

impl Default for IsometricCamera3dBundle {
    fn default() -> Self {
        Self {
            fixed_camera: FixedCamera3dBundle::new(
                ISOMETRIC_CAMERA_3D_TRANSLATION,
                ISOMETRIC_CAMERA_3D_TARGET,
                Vec3::Y,
                default(),
            ),
            marker: IsometricCamera3dMarker,
        }
    }
}
