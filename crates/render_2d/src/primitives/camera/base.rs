use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub(in crate::primitives::camera) struct BaseCamera2dConfig {
    pub order: isize,
    pub layer: usize,
    pub z: f32,
    pub clear_color: ClearColorConfig,
}

#[derive(Bundle)]
pub(in crate::primitives::camera) struct BaseCamera2dBundle {
    camera_2d: Camera2d,
    camera: Camera,
    render_layers: RenderLayers,
    transform: Transform,
}

impl BaseCamera2dBundle {
    pub(in crate::primitives::camera) fn new(config: BaseCamera2dConfig) -> Self {
        Self {
            camera_2d: Camera2d,
            camera: Camera {
                order: config.order,
                clear_color: config.clear_color,
                ..default()
            },
            render_layers: RenderLayers::layer(config.layer),
            transform: Transform::from_xyz(0.0, 0.0, config.z),
        }
    }
}
