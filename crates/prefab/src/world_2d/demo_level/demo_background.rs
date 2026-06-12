use bevy::prelude::*;
use render_2d::background::DemoBackgroundLayer2dBundle;

use crate::Prefab;

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
                    DemoBackgroundLayer2dBundle::new(
                        Color::srgb(0.18, 0.28, 0.38),
                        Vec2::new(2400.0, 1400.0),
                        -20.0,
                        Vec2::new(0.15, 0.08),
                    ),
                    DemoBackgroundLayer2dBundle::new(
                        Color::srgba(0.16, 0.42, 0.30, 0.86),
                        Vec2::new(2200.0, 900.0),
                        -10.0,
                        Vec2::new(0.35, 0.14),
                    ),
                ],
            ))
            .id()
    }
}
