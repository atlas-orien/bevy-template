//! 单方向太阳光预设。

use bevy::prelude::*;

use crate::primitives::lights::base::DirectionalLight3dBundle;
use crate::primitives::transforms::LookAtTransform3d;

const SUN_LIGHT_3D_ILLUMINANCE: f32 = 18_000.0;
const SUN_LIGHT_3D_TRANSLATION: Vec3 = Vec3::new(-2.5, 5.0, 4.5);
const SUN_LIGHT_3D_TARGET: Vec3 = Vec3::ZERO;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct SunLight3dMarker;

#[derive(Bundle)]
pub struct SunLight3dBundle {
    light: DirectionalLight3dBundle,
    marker: SunLight3dMarker,
}

impl Default for SunLight3dBundle {
    fn default() -> Self {
        Self {
            light: DirectionalLight3dBundle::new(
                DirectionalLight {
                    illuminance: SUN_LIGHT_3D_ILLUMINANCE,
                    shadows_enabled: true,
                    ..default()
                },
                LookAtTransform3d::new(SUN_LIGHT_3D_TRANSLATION, SUN_LIGHT_3D_TARGET, Vec3::Y)
                    .into(),
            ),
            marker: SunLight3dMarker,
        }
    }
}
