use bevy::prelude::*;

use super::layers::demo_background_layers;
use crate::background::layered::LayeredBackground2d;

pub struct DemoBackground2d {
    background: LayeredBackground2d,
}

impl Default for DemoBackground2d {
    fn default() -> Self {
        Self::new()
    }
}

impl DemoBackground2d {
    pub fn into_bundle(self) -> impl Bundle {
        self.background.into_bundle()
    }
}

impl From<LayeredBackground2d> for DemoBackground2d {
    fn from(background: LayeredBackground2d) -> Self {
        Self { background }
    }
}

impl DemoBackground2d {
    pub fn new() -> Self {
        LayeredBackground2d::new(demo_background_layers()).into()
    }
}
