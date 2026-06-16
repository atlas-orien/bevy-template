//! Demo 视差背景 prefab。

use bevy::prelude::*;
use render_2d::products::background::DemoBackground2d;

use crate::Prefab;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoBackgroundRoot;

pub struct DemoBackgroundPrefab;

impl Prefab for DemoBackgroundPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands
            .spawn((
                DemoBackgroundRoot,
                DemoBackground2d::default().into_bundle(),
            ))
            .id()
    }
}
