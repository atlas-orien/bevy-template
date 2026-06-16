use bevy::prelude::*;

use crate::background::layered::BackgroundLayer2d;

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

pub(in crate::background::demo) fn demo_background_layers() -> [BackgroundLayer2d; 2] {
    [
        BackgroundLayer2d::color(
            DEMO_SKY_BACKGROUND_COLOR,
            DEMO_SKY_BACKGROUND_SIZE,
            DEMO_SKY_BACKGROUND_Z,
            DEMO_SKY_PARALLAX_FACTOR,
        ),
        BackgroundLayer2d::color(
            DEMO_FOREST_BACKGROUND_COLOR,
            DEMO_FOREST_BACKGROUND_SIZE,
            DEMO_FOREST_BACKGROUND_Z,
            DEMO_FOREST_PARALLAX_FACTOR,
        ),
    ]
}
