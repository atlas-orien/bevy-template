//! Demo 视差背景产品。

use bevy::prelude::*;

use crate::primitives::layers::{LayerStack2d, RenderLayer2d};

const DEMO_SKY_BACKGROUND_COLOR: Color = Color::srgb(0.18, 0.28, 0.38);
const DEMO_SKY_BACKGROUND_SIZE: Vec2 = Vec2::new(2400.0, 1400.0);
const DEMO_SKY_BACKGROUND_Z: f32 = -20.0;
const DEMO_SKY_PARALLAX_FACTOR: Vec2 = Vec2::new(0.15, 0.08);

const DEMO_FOREST_BACKGROUND_COLOR: Color = Color::srgba(0.16, 0.42, 0.30, 0.86);
const DEMO_FOREST_BACKGROUND_SIZE: Vec2 = Vec2::new(2200.0, 900.0);
const DEMO_FOREST_BACKGROUND_Z: f32 = -10.0;
const DEMO_FOREST_PARALLAX_FACTOR: Vec2 = Vec2::new(0.35, 0.14);

pub struct DemoBackground2d {
    stack: LayerStack2d,
}

impl Default for DemoBackground2d {
    fn default() -> Self {
        Self::new()
    }
}

impl DemoBackground2d {
    pub fn into_bundle(self) -> impl Bundle {
        self.stack.into_bundle()
    }

    pub fn new() -> Self {
        Self {
            stack: LayerStack2d::new([
                RenderLayer2d::color(
                    DEMO_SKY_BACKGROUND_COLOR,
                    DEMO_SKY_BACKGROUND_SIZE,
                    DEMO_SKY_BACKGROUND_Z,
                    DEMO_SKY_PARALLAX_FACTOR,
                ),
                RenderLayer2d::color(
                    DEMO_FOREST_BACKGROUND_COLOR,
                    DEMO_FOREST_BACKGROUND_SIZE,
                    DEMO_FOREST_BACKGROUND_Z,
                    DEMO_FOREST_PARALLAX_FACTOR,
                ),
            ]),
        }
    }
}
