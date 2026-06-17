use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub(in crate::primitives::camera) struct BaseCamera3dConfig {
    pub order: isize,
    pub layer: usize,
    pub translation: Vec3,
    pub target: Vec3,
    pub clear_color: ClearColorConfig,
}

#[derive(Bundle)]
pub(in crate::primitives::camera) struct BaseCamera3dBundle {
    camera_3d: Camera3d,
    camera: Camera,
    render_layers: RenderLayers,
    transform: Transform,
}

impl BaseCamera3dBundle {
    pub(in crate::primitives::camera) fn new(config: BaseCamera3dConfig) -> Self {
        Self {
            camera_3d: Camera3d::default(),
            camera: Camera {
                order: config.order,
                clear_color: config.clear_color,
                ..default()
            },
            render_layers: RenderLayers::layer(config.layer),
            transform: Transform::from_translation(config.translation)
                .looking_at(config.target, Vec3::Y),
        }
    }
}
