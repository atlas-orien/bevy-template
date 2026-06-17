//! 跟随目标的 2D 场景相机。

use bevy::prelude::*;

use super::FixedCamera2dBundle;

const FOLLOW_CAMERA_2D_SMOOTHNESS: f32 = 12.0;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct FollowCameraTarget2dMarker;

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

pub(in crate::primitives::camera) fn follow_camera_system(
    time: Res<Time>,
    target: Query<&GlobalTransform, With<FollowCameraTarget2dMarker>>,
    mut cameras: Query<(&FollowCamera2d, &mut Transform)>,
) {
    let Ok(target) = target.single() else {
        return;
    };
    let target_translation = target.translation();

    for (follow, mut camera_transform) in &mut cameras {
        let t = 1.0 - (-follow.smoothness * time.delta_secs()).exp();
        camera_transform.translation.x =
            camera_transform.translation.x.lerp(target_translation.x, t);
        camera_transform.translation.y =
            camera_transform.translation.y.lerp(target_translation.y, t);
    }
}
