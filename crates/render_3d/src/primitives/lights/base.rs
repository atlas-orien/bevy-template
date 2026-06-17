use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DirectionalLight3dMarker;

#[derive(Bundle)]
pub struct DirectionalLight3dBundle {
    marker: DirectionalLight3dMarker,
    light: DirectionalLight,
    transform: Transform,
}

impl DirectionalLight3dBundle {
    pub fn new(light: DirectionalLight, transform: Transform) -> Self {
        Self {
            marker: DirectionalLight3dMarker,
            light,
            transform,
        }
    }
}

impl Default for DirectionalLight3dBundle {
    fn default() -> Self {
        Self::new(DirectionalLight::default(), Transform::default())
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
    pub fn new(light: PointLight, transform: Transform) -> Self {
        Self {
            marker: PointLight3dMarker,
            light,
            transform,
        }
    }
}

impl Default for PointLight3dBundle {
    fn default() -> Self {
        Self::new(PointLight::default(), Transform::default())
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
    pub fn new(light: SpotLight, transform: Transform) -> Self {
        Self {
            marker: SpotLight3dMarker,
            light,
            transform,
        }
    }
}

impl Default for SpotLight3dBundle {
    fn default() -> Self {
        Self::new(SpotLight::default(), Transform::default())
    }
}
