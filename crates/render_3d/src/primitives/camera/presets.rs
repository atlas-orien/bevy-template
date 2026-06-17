//! 可直接实例化的 3D camera preset。

use bevy::prelude::*;

use super::base::{BaseCamera3dBundle, BaseCamera3dConfig};

const FIXED_CAMERA_3D_ORDER: isize = 0;
const FIXED_CAMERA_3D_LAYER: usize = 0;
const FIXED_CAMERA_3D_TRANSLATION: Vec3 = Vec3::new(6.0, 5.0, 8.0);
const FIXED_CAMERA_3D_TARGET: Vec3 = Vec3::ZERO;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct FixedCamera3dMarker;

#[derive(Bundle)]
pub struct FixedCamera3dBundle {
    camera: BaseCamera3dBundle,
    marker: FixedCamera3dMarker,
}

impl Default for FixedCamera3dBundle {
    fn default() -> Self {
        Self::new(FIXED_CAMERA_3D_TRANSLATION, FIXED_CAMERA_3D_TARGET)
    }
}

impl FixedCamera3dBundle {
    pub fn new(translation: Vec3, target: Vec3) -> Self {
        Self {
            camera: BaseCamera3dBundle::new(BaseCamera3dConfig {
                order: FIXED_CAMERA_3D_ORDER,
                layer: FIXED_CAMERA_3D_LAYER,
                translation,
                target,
                clear_color: default(),
            }),
            marker: FixedCamera3dMarker,
        }
    }
}
