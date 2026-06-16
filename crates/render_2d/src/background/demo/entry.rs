use bevy::prelude::*;

use super::layers::{DemoBackground2dBundle, DemoBackgroundLayers2d};

#[derive(Default)]
pub struct DemoBackground2d {
    pub bundle: DemoBackground2dBundle,
    layers: DemoBackgroundLayers2d,
}

impl DemoBackground2d {
    pub fn into_bundle(self) -> impl Bundle {
        (self.bundle, self.layers.into_children())
    }
}
