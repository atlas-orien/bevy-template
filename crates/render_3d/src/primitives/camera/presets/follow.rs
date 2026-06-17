//! 跟随目标的 3D 场景相机。

use bevy::prelude::*;

use super::FixedCamera3dBundle;

const FOLLOW_CAMERA_3D_OFFSET: Vec3 = Vec3::new(0.0, 3.8, 7.0);
const FOLLOW_CAMERA_3D_TARGET_OFFSET: Vec3 = Vec3::new(0.0, 1.0, 0.0);
const FOLLOW_CAMERA_3D_SMOOTHNESS: f32 = 8.0;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct FollowCameraTarget3dMarker;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct FollowCamera3d {
    pub offset: Vec3,
    pub target_offset: Vec3,
    pub up: Vec3,
    pub smoothness: f32,
}

impl Default for FollowCamera3d {
    fn default() -> Self {
        Self {
            offset: FOLLOW_CAMERA_3D_OFFSET,
            target_offset: FOLLOW_CAMERA_3D_TARGET_OFFSET,
            up: Vec3::Y,
            smoothness: FOLLOW_CAMERA_3D_SMOOTHNESS,
        }
    }
}

#[derive(Bundle, Default)]
pub struct FollowCamera3dBundle {
    fixed_camera: FixedCamera3dBundle,
    follow: FollowCamera3d,
}

pub(in crate::primitives::camera) fn follow_camera_3d_system(
    time: Res<Time>,
    target: Query<&GlobalTransform, With<FollowCameraTarget3dMarker>>,
    mut cameras: Query<(&FollowCamera3d, &mut Transform)>,
) {
    let Ok(target) = target.single() else {
        return;
    };

    let target_translation = target.translation();
    for (follow, mut camera_transform) in &mut cameras {
        let look_target = target_translation + follow.target_offset;
        let desired_translation = look_target + follow.offset;
        let t = 1.0 - (-follow.smoothness * time.delta_secs()).exp();

        camera_transform.translation = camera_transform.translation.lerp(desired_translation, t);
        camera_transform.look_at(look_target, follow.up);
    }
}
