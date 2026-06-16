use bevy::prelude::*;

pub(in crate::background::demo) const DEMO_SKY_BACKGROUND_COLOR: Color =
    Color::srgb(0.18, 0.28, 0.38);
pub(in crate::background::demo) const DEMO_SKY_BACKGROUND_SIZE: Vec2 = Vec2::new(2400.0, 1400.0);
pub(in crate::background::demo) const DEMO_SKY_BACKGROUND_Z: f32 = -20.0;
pub(in crate::background::demo) const DEMO_SKY_PARALLAX_FACTOR: Vec2 = Vec2::new(0.15, 0.08);
pub(in crate::background::demo) const DEMO_FOREST_BACKGROUND_COLOR: Color =
    Color::srgba(0.16, 0.42, 0.30, 0.86);
pub(in crate::background::demo) const DEMO_FOREST_BACKGROUND_SIZE: Vec2 = Vec2::new(2200.0, 900.0);
pub(in crate::background::demo) const DEMO_FOREST_BACKGROUND_Z: f32 = -10.0;
pub(in crate::background::demo) const DEMO_FOREST_PARALLAX_FACTOR: Vec2 = Vec2::new(0.35, 0.14);

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub(in crate::background) struct DemoBackgroundLayer2dMarker;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoBackgroundRoot2d;

#[derive(Component, Debug, Clone, Copy, Default, PartialEq)]
pub(in crate::background) struct DemoParallaxBackgroundLayer2d {
    pub(in crate::background) speed: Vec2,
}

#[derive(Bundle, Default)]
pub struct DemoBackground2dBundle {
    marker: DemoBackgroundRoot2d,
    transform: Transform,
    visibility: Visibility,
}

pub(in crate::background::demo) struct DemoBackgroundLayers2d {
    sky: DemoBackgroundLayer2d,
    forest: DemoBackgroundLayer2d,
}

impl Default for DemoBackgroundLayers2d {
    fn default() -> Self {
        Self {
            sky: DemoBackgroundLayer2d::new(
                DEMO_SKY_BACKGROUND_COLOR,
                DEMO_SKY_BACKGROUND_SIZE,
                DEMO_SKY_BACKGROUND_Z,
                DEMO_SKY_PARALLAX_FACTOR,
            ),
            forest: DemoBackgroundLayer2d::new(
                DEMO_FOREST_BACKGROUND_COLOR,
                DEMO_FOREST_BACKGROUND_SIZE,
                DEMO_FOREST_BACKGROUND_Z,
                DEMO_FOREST_PARALLAX_FACTOR,
            ),
        }
    }
}

impl DemoBackgroundLayers2d {
    pub(in crate::background::demo) fn into_children(self) -> impl Bundle {
        children![self.sky.into_bundle(), self.forest.into_bundle()]
    }
}

struct DemoBackgroundLayer2d {
    color: Color,
    size: Vec2,
    z: f32,
    parallax_speed: Vec2,
}

impl DemoBackgroundLayer2d {
    fn new(color: Color, size: Vec2, z: f32, parallax_speed: Vec2) -> Self {
        Self {
            color,
            size,
            z,
            parallax_speed,
        }
    }

    fn into_bundle(self) -> impl Bundle {
        (
            DemoBackgroundLayer2dMarker,
            DemoParallaxBackgroundLayer2d {
                speed: self.parallax_speed,
            },
            Sprite {
                color: self.color,
                custom_size: Some(self.size),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, self.z),
        )
    }
}
