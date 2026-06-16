use bevy::prelude::*;

use crate::camera::FixedCamera2dBundle;

const FOLLOW_CAMERA_2D_SMOOTHNESS: f32 = 12.0;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct FollowCamera2d {
    pub smoothness: f32,
}

impl Default for FollowCamera2d {
    fn default() -> Self {
        Self {
            smoothness: FOLLOW_CAMERA_2D_SMOOTHNESS,
        }
    }
}

#[derive(Bundle, Default)]
pub struct FollowCamera2dBundle {
    fixed_camera: FixedCamera2dBundle,
    follow: FollowCamera2d,
}
