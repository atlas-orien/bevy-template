//! Demo 视差背景产品。

use bevy::prelude::*;

use crate::primitives::layers::{LayerStack2dBundle, LayerStack2dRootBundle, RenderLayer2d};

const DEMO_SKY_BACKGROUND_COLOR: Color = Color::srgb(0.18, 0.28, 0.38);
const DEMO_SKY_BACKGROUND_SIZE: Vec2 = Vec2::new(2400.0, 1400.0);
const DEMO_SKY_BACKGROUND_Z: f32 = -20.0;
const DEMO_SKY_PARALLAX_FACTOR: Vec2 = Vec2::new(0.15, 0.08);

const DEMO_FOREST_BACKGROUND_COLOR: Color = Color::srgba(0.16, 0.42, 0.30, 0.86);
const DEMO_FOREST_BACKGROUND_SIZE: Vec2 = Vec2::new(2200.0, 900.0);
const DEMO_FOREST_BACKGROUND_Z: f32 = -10.0;
const DEMO_FOREST_PARALLAX_FACTOR: Vec2 = Vec2::new(0.35, 0.14);

pub struct DemoBackground2d;

#[derive(Bundle)]
#[bundle(ignore_from_components)]
pub struct DemoBackground2dBundle {
    root: LayerStack2dRootBundle,
    layers: crate::primitives::layers::LayerStackChildren2dBundle,
}

impl DemoBackground2d {
    pub fn bundle() -> DemoBackground2dBundle {
        DemoBackground2dBundle::default()
    }
}

impl Default for DemoBackground2dBundle {
    fn default() -> Self {
        let stack = LayerStack2dBundle::new([
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
        ]);

        Self {
            root: stack.root,
            layers: stack.children,
        }
    }
}
