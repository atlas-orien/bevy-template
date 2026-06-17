use bevy::prelude::*;

use super::{ParallaxLayer2d, RenderLayer2dMarker};
use crate::primitives::camera::SceneCamera2dMarker;

pub(super) fn parallax_layer_system(
    camera: Query<&Transform, (With<SceneCamera2dMarker>, Without<RenderLayer2dMarker>)>,
    mut layers: Query<(&ParallaxLayer2d, &mut Transform), With<RenderLayer2dMarker>>,
) {
    let Ok(camera) = camera.single() else {
        return;
    };
    let camera_translation = camera.translation.truncate();

    for (parallax, mut transform) in &mut layers {
        let offset = camera_translation * parallax.speed;
        transform.translation.x = offset.x;
        transform.translation.y = offset.y;
    }
}
