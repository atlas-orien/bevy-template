use bevy::ecs::spawn::SpawnIter;
use bevy::prelude::*;

use super::entry::RenderLayer2d;

#[derive(Bundle, Default)]
pub struct LayerStack2dBundle {
    marker: LayerStack2dRoot,
    transform: Transform,
    visibility: Visibility,
}

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct LayerStack2dRoot;

#[derive(Default)]
pub(in crate::layers) struct LayerStackChildren2d {
    layers: Vec<RenderLayer2d>,
}

impl LayerStackChildren2d {
    pub(in crate::layers) fn new(layers: impl IntoIterator<Item = RenderLayer2d>) -> Self {
        Self {
            layers: layers.into_iter().collect(),
        }
    }

    pub(in crate::layers) fn into_children(self) -> impl Bundle {
        Children::spawn(SpawnIter(
            self.layers.into_iter().map(RenderLayer2d::into_bundle),
        ))
    }
}
