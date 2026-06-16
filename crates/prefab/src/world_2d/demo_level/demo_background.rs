//! Demo 视差背景 prefab。

use bevy::prelude::*;
use render_2d::background::DemoBackgroundLayer2d;

use crate::Prefab;

const DEMO_SKY_BACKGROUND_COLOR: Color = Color::srgb(0.18, 0.28, 0.38);
const DEMO_SKY_BACKGROUND_SIZE: Vec2 = Vec2::new(2400.0, 1400.0);
const DEMO_SKY_BACKGROUND_Z: f32 = -20.0;
const DEMO_SKY_PARALLAX_FACTOR: Vec2 = Vec2::new(0.15, 0.08);
const DEMO_FOREST_BACKGROUND_COLOR: Color = Color::srgba(0.16, 0.42, 0.30, 0.86);
const DEMO_FOREST_BACKGROUND_SIZE: Vec2 = Vec2::new(2200.0, 900.0);
const DEMO_FOREST_BACKGROUND_Z: f32 = -10.0;
const DEMO_FOREST_PARALLAX_FACTOR: Vec2 = Vec2::new(0.35, 0.14);

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoBackgroundRoot;

pub struct DemoBackgroundPrefab;

impl Prefab for DemoBackgroundPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands
            .spawn((
                DemoBackgroundRoot,
                Transform::default(),
                Visibility::default(),
                children![
                    DemoBackgroundLayer2d::new(
                        DEMO_SKY_BACKGROUND_COLOR,
                        DEMO_SKY_BACKGROUND_SIZE,
                        DEMO_SKY_BACKGROUND_Z,
                        DEMO_SKY_PARALLAX_FACTOR,
                    )
                    .into_bundle(),
                    DemoBackgroundLayer2d::new(
                        DEMO_FOREST_BACKGROUND_COLOR,
                        DEMO_FOREST_BACKGROUND_SIZE,
                        DEMO_FOREST_BACKGROUND_Z,
                        DEMO_FOREST_PARALLAX_FACTOR,
                    )
                    .into_bundle(),
                ],
            ))
            .id()
    }
}
