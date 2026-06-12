use bevy::prelude::*;

use super::DemoWorldCamera2d;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct DemoCameraFollow {
    pub smoothness: f32,
}

impl Default for DemoCameraFollow {
    fn default() -> Self {
        Self { smoothness: 7.0 }
    }
}

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoCameraFollowTarget;

pub fn demo_camera_follow_system(
    time: Res<Time>,
    target: Query<&GlobalTransform, With<DemoCameraFollowTarget>>,
    mut cameras: Query<(&DemoCameraFollow, &mut Transform), With<DemoWorldCamera2d>>,
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
