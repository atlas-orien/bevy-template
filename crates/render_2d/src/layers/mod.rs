//! 通用 2D layer 组合能力，供 background、environment、screens 等复合表现使用。

mod plugin;
mod systems;

use bevy::ecs::spawn::SpawnIter;
use bevy::prelude::*;

use crate::images::StaticImage2d;

pub use plugin::Layers2dPlugin;

#[derive(Default)]
pub struct LayerStack2d {
    pub bundle: LayerStack2dBundle,
    layers: LayerStackChildren2d,
}

impl LayerStack2d {
    pub fn new(layers: impl IntoIterator<Item = RenderLayer2d>) -> Self {
        Self {
            bundle: LayerStack2dBundle::default(),
            layers: LayerStackChildren2d::new(layers),
        }
    }

    pub fn into_bundle(self) -> impl Bundle {
        (self.bundle, self.layers.into_children())
    }
}

#[derive(Bundle, Default)]
pub struct LayerStack2dBundle {
    marker: LayerStack2dRoot,
    transform: Transform,
    visibility: Visibility,
}

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct LayerStack2dRoot;

pub struct RenderLayer2d {
    color: Color,
    image: Option<Handle<Image>>,
    size: Vec2,
    z: f32,
    parallax_speed: Vec2,
}

impl RenderLayer2d {
    pub fn color(color: Color, size: Vec2, z: f32, parallax_speed: Vec2) -> Self {
        Self {
            color,
            image: None,
            size,
            z,
            parallax_speed,
        }
    }

    pub fn image(image: Handle<Image>, size: Vec2, z: f32, parallax_speed: Vec2) -> Self {
        Self {
            color: Color::WHITE,
            image: Some(image),
            size,
            z,
            parallax_speed,
        }
    }

    pub(in crate::layers) fn into_bundle(self) -> impl Bundle {
        let image = match self.image {
            Some(image) => StaticImage2d::image(image, self.size, self.z),
            None => StaticImage2d::color(self.color, self.size, self.z),
        };

        (
            RenderLayer2dMarker,
            ParallaxLayer2d {
                speed: self.parallax_speed,
            },
            image.into_bundle(),
        )
    }
}

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub(in crate::layers) struct RenderLayer2dMarker;

#[derive(Component, Debug, Clone, Copy, Default, PartialEq)]
pub(in crate::layers) struct ParallaxLayer2d {
    pub(in crate::layers) speed: Vec2,
}

#[derive(Default)]
struct LayerStackChildren2d {
    layers: Vec<RenderLayer2d>,
}

impl LayerStackChildren2d {
    fn new(layers: impl IntoIterator<Item = RenderLayer2d>) -> Self {
        Self {
            layers: layers.into_iter().collect(),
        }
    }

    fn into_children(self) -> impl Bundle {
        Children::spawn(SpawnIter(
            self.layers.into_iter().map(RenderLayer2d::into_bundle),
        ))
    }
}
