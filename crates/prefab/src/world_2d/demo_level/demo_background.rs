//! Demo 视差背景 prefab。

use bevy::prelude::*;
use render_2d::products::background::DemoBackground2dBundle;

use crate::Prefab;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoBackgroundRoot;

pub struct DemoBackgroundPrefab;

#[derive(Bundle, Default)]
struct DemoBackgroundRootBundle {
    root: DemoBackgroundRoot,
}

impl Prefab for DemoBackgroundPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands
            .spawn(DemoBackgroundRootBundle::default())
            .insert(DemoBackground2dBundle::default())
            .id()
    }
}
