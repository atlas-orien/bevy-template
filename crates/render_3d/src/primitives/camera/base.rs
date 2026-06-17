use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;

#[derive(Clone)]
pub(in crate::primitives::camera) struct BaseCamera3dConfig {
    pub camera_3d: Camera3d,
    pub camera: Camera,
    pub projection: Projection,
    pub render_layers: RenderLayers,
    pub transform: Transform,
}

impl Default for BaseCamera3dConfig {
    fn default() -> Self {
        Self {
            camera_3d: Camera3d::default(),
            camera: Camera::default(),
            projection: Projection::Perspective(PerspectiveProjection::default()),
            render_layers: RenderLayers::layer(0),
            transform: Transform::default(),
        }
    }
}

#[derive(Bundle)]
pub(in crate::primitives::camera) struct BaseCamera3dBundle {
    camera_3d: Camera3d,
    camera: Camera,
    projection: Projection,
    render_layers: RenderLayers,
    transform: Transform,
}

impl BaseCamera3dBundle {
    pub(in crate::primitives::camera) fn new(config: BaseCamera3dConfig) -> Self {
        Self {
            camera_3d: config.camera_3d,
            camera: config.camera,
            projection: config.projection,
            render_layers: config.render_layers,
            transform: config.transform,
        }
    }
}
