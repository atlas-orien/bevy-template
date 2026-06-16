//! Demo pixel 表现：把视觉 Transform 对齐到像素格。

use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct DemoPixelSnap2d {
    pixels_per_unit: f32,
}

impl Default for DemoPixelSnap2d {
    fn default() -> Self {
        Self {
            pixels_per_unit: 1.0,
        }
    }
}

impl DemoPixelSnap2d {
    pub fn new(pixels_per_unit: f32) -> Self {
        Self {
            pixels_per_unit: pixels_per_unit.max(1.0),
        }
    }
}

pub(super) fn demo_pixel_snap_system(mut visuals: Query<(&DemoPixelSnap2d, &mut Transform)>) {
    for (snap, mut transform) in &mut visuals {
        transform.translation.x =
            (transform.translation.x * snap.pixels_per_unit).round() / snap.pixels_per_unit;
        transform.translation.y =
            (transform.translation.y * snap.pixels_per_unit).round() / snap.pixels_per_unit;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_pixel_snap_rounds_translation_to_pixel_grid() {
        let mut app = App::new();
        app.add_systems(Update, demo_pixel_snap_system);
        let entity = app
            .world_mut()
            .spawn((
                DemoPixelSnap2d::new(2.0),
                Transform::from_xyz(1.24, 2.76, 9.0),
            ))
            .id();

        app.update();

        let transform = app
            .world()
            .get::<Transform>(entity)
            .expect("transform should stay");
        assert_eq!(transform.translation, Vec3::new(1.0, 3.0, 9.0));
    }
}
