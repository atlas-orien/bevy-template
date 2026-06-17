mod plugin;

use bevy::prelude::*;

pub use plugin::Lights3dPlugin;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DirectionalLight3dMarker;

#[derive(Bundle)]
pub struct DirectionalLight3dBundle {
    marker: DirectionalLight3dMarker,
    light: DirectionalLight,
    transform: Transform,
}

impl Default for DirectionalLight3dBundle {
    fn default() -> Self {
        Self::new(DirectionalLight {
            illuminance: 35_000.0,
            shadows_enabled: true,
            ..default()
        })
    }
}

impl DirectionalLight3dBundle {
    pub fn new(light: DirectionalLight) -> Self {
        Self {
            marker: DirectionalLight3dMarker,
            light,
            transform: Transform::from_xyz(-2.5, 5.0, 4.5).looking_at(Vec3::ZERO, Vec3::Y),
        }
    }
}

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct PointLight3dMarker;

#[derive(Bundle)]
pub struct PointLight3dBundle {
    marker: PointLight3dMarker,
    light: PointLight,
    transform: Transform,
}

impl PointLight3dBundle {
    pub fn new(light: PointLight, translation: Vec3) -> Self {
        Self {
            marker: PointLight3dMarker,
            light,
            transform: Transform::from_translation(translation),
        }
    }
}

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct SpotLight3dMarker;

#[derive(Bundle)]
pub struct SpotLight3dBundle {
    marker: SpotLight3dMarker,
    light: SpotLight,
    transform: Transform,
}

impl SpotLight3dBundle {
    pub fn new(light: SpotLight, translation: Vec3, target: Vec3) -> Self {
        Self {
            marker: SpotLight3dMarker,
            light,
            transform: Transform::from_translation(translation).looking_at(target, Vec3::Y),
        }
    }
}
