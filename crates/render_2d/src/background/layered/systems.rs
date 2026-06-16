use bevy::prelude::*;

use super::entry::{BackgroundLayer2dMarker, ParallaxBackgroundLayer2d};
use crate::camera::DemoWorldCamera2dMarker;

pub(in crate::background) fn layered_background_parallax_system(
    camera: Query<
        &Transform,
        (
            With<DemoWorldCamera2dMarker>,
            Without<BackgroundLayer2dMarker>,
        ),
    >,
    mut backgrounds: Query<
        (&ParallaxBackgroundLayer2d, &mut Transform),
        With<BackgroundLayer2dMarker>,
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
