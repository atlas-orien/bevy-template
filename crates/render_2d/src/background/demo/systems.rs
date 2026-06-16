use bevy::prelude::*;

use super::layers::{DemoBackgroundLayer2dMarker, DemoParallaxBackgroundLayer2d};
use crate::camera::DemoWorldCamera2dMarker;

pub(in crate::background) fn demo_parallax_background_system(
    camera: Query<
        &Transform,
        (
            With<DemoWorldCamera2dMarker>,
            Without<DemoBackgroundLayer2dMarker>,
        ),
    >,
    mut backgrounds: Query<
        (&DemoParallaxBackgroundLayer2d, &mut Transform),
        With<DemoBackgroundLayer2dMarker>,
    >,
) {
    let Ok(camera) = camera.single() else {
        return;
    };
    let camera_translation = camera.translation.truncate();

    for (parallax, mut transform) in &mut backgrounds {
        let offset = camera_translation * parallax.speed;
        transform.translation.x = offset.x;
        transform.translation.y = offset.y;
    }
}
