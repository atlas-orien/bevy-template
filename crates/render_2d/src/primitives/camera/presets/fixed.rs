//! 固定不动的 2D 场景相机。

use bevy::prelude::*;

use crate::primitives::camera::base::{BaseCamera2dBundle, BaseCamera2dConfig};
use crate::primitives::markers::SceneCamera2d;

const FIXED_CAMERA_2D_ORDER: isize = 0;
const FIXED_CAMERA_2D_LAYER: usize = 0;
const FIXED_CAMERA_2D_Z: f32 = 1000.0;

#[derive(Bundle)]
pub struct FixedCamera2dBundle {
    camera: BaseCamera2dBundle,
    marker: SceneCamera2d,
}

impl Default for FixedCamera2dBundle {
    fn default() -> Self {
        Self {
            camera: BaseCamera2dBundle::new(BaseCamera2dConfig {
                order: FIXED_CAMERA_2D_ORDER,
                layer: FIXED_CAMERA_2D_LAYER,
                z: FIXED_CAMERA_2D_Z,
                clear_color: default(),
            }),
            marker: SceneCamera2d,
        }
    }
}
