//! 围绕目标点环绕的 3D 场景相机。

use bevy::prelude::*;

use super::FixedCamera3dBundle;

const ORBIT_CAMERA_3D_RADIUS: f32 = 7.5;
const ORBIT_CAMERA_3D_YAW: f32 = -0.65;
const ORBIT_CAMERA_3D_PITCH: f32 = 0.45;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct OrbitCamera3dMarker;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct OrbitCamera3d {
    pub target: Vec3,
    pub radius: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub up: Vec3,
}

impl Default for OrbitCamera3d {
    fn default() -> Self {
        Self {
            target: Vec3::ZERO,
            radius: ORBIT_CAMERA_3D_RADIUS,
            yaw: ORBIT_CAMERA_3D_YAW,
            pitch: ORBIT_CAMERA_3D_PITCH,
            up: Vec3::Y,
        }
    }
}

#[derive(Bundle)]
pub struct OrbitCamera3dBundle {
    fixed_camera: FixedCamera3dBundle,
    orbit: OrbitCamera3d,
    marker: OrbitCamera3dMarker,
}

impl Default for OrbitCamera3dBundle {
    fn default() -> Self {
        let orbit = OrbitCamera3d::default();
        Self {
            fixed_camera: FixedCamera3dBundle::new(
                orbit_translation(orbit),
                orbit.target,
                orbit.up,
                default(),
            ),
            orbit,
            marker: OrbitCamera3dMarker,
        }
    }
}

pub(in crate::primitives::camera) fn orbit_camera_3d_system(
    mut cameras: Query<(&OrbitCamera3d, &mut Transform)>,
) {
    for (orbit, mut camera_transform) in &mut cameras {
        camera_transform.translation = orbit_translation(*orbit);
        camera_transform.look_at(orbit.target, orbit.up);
    }
}

fn orbit_translation(orbit: OrbitCamera3d) -> Vec3 {
    let horizontal_radius = orbit.radius * orbit.pitch.cos();
    orbit.target
        + Vec3::new(
            horizontal_radius * orbit.yaw.sin(),
            orbit.radius * orbit.pitch.sin(),
            horizontal_radius * orbit.yaw.cos(),
        )
}
