use bevy::prelude::*;

use super::layers::demo_background_layers;
use crate::layers::LayerStack2d;

pub struct DemoBackground2d {
    layers: LayerStack2d,
}

impl Default for DemoBackground2d {
    fn default() -> Self {
        Self::new()
    }
}

impl DemoBackground2d {
    pub fn into_bundle(self) -> impl Bundle {
        self.layers.into_bundle()
    }
}

impl From<LayerStack2d> for DemoBackground2d {
    fn from(layers: LayerStack2d) -> Self {
        Self { layers }
    }
}

impl DemoBackground2d {
    pub fn new() -> Self {
        LayerStack2d::new(demo_background_layers()).into()
    }
}
