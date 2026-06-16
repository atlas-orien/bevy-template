use bevy::ecs::spawn::SpawnIter;
use bevy::prelude::*;

use super::entry::BackgroundLayer2d;

#[derive(Bundle, Default)]
pub struct LayeredBackground2dBundle {
    marker: LayeredBackgroundRoot2d,
    transform: Transform,
    visibility: Visibility,
}

#[derive(Default)]
pub(in crate::background::layered) struct LayeredBackgroundLayers2d {
    layers: Vec<BackgroundLayer2d>,
}

impl LayeredBackgroundLayers2d {
    pub(in crate::background::layered) fn new(
        layers: impl IntoIterator<Item = BackgroundLayer2d>,
    ) -> Self {
        Self {
            layers: layers.into_iter().collect(),
        }
    }

    pub(in crate::background::layered) fn into_children(self) -> impl Bundle {
        Children::spawn(SpawnIter(
            self.layers.into_iter().map(BackgroundLayer2d::into_bundle),
        ))
    }
}

use super::entry::LayeredBackgroundRoot2d;
