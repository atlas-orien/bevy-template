//! Demo 视差背景 prefab。

use bevy::prelude::*;
use render_2d::products::background::DemoBackground2dBundle;

use crate::Prefab;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
struct DemoBackgroundRoot;

#[derive(Default)]
pub struct DemoBackgroundPrefab;

#[derive(Bundle, Default)]
#[bundle(ignore_from_components)]
struct DemoBackgroundBundle {
    root: DemoBackgroundRoot,
    visual: DemoBackground2dBundle,
}

impl Prefab for DemoBackgroundPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands.spawn(DemoBackgroundBundle::default()).id()
    }
}
