use bevy::prelude::*;

use super::FollowCamera2d;
use crate::camera::FollowCameraTarget2d;

pub(in crate::camera) fn follow_camera_system(
    time: Res<Time>,
    target: Query<&GlobalTransform, With<FollowCameraTarget2d>>,
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
