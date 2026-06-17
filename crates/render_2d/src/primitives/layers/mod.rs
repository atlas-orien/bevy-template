//! 通用 2D layer 组合能力，供 background、environment、screens 等复合表现使用。

mod plugin;
mod systems;

use bevy::ecs::spawn::SpawnIter;
use bevy::ecs::spawn::SpawnRelatedBundle;
use bevy::prelude::*;

use crate::primitives::images::StaticImage2d;
use crate::primitives::images::StaticImage2dBundle;

pub use plugin::Layers2dPlugin;

#[derive(Default)]
pub struct LayerStack2d {
    pub bundle: LayerStack2dBundle,
}

impl LayerStack2d {
    pub fn new(layers: impl IntoIterator<Item = RenderLayer2d>) -> Self {
        Self {
            bundle: LayerStack2dBundle::new(layers),
        }
    }
}

#[derive(Bundle)]
#[bundle(ignore_from_components)]
pub struct LayerStack2dBundle {
    pub(crate) root: LayerStack2dRootBundle,
    pub(crate) children: LayerStackChildren2dBundle,
}

impl LayerStack2dBundle {
    pub fn new(layers: impl IntoIterator<Item = RenderLayer2d>) -> Self {
        let layers = layers.into_iter().collect::<Vec<_>>();
        Self {
            root: LayerStack2dRootBundle::default(),
            children: Children::spawn(SpawnIter(
                layers
                    .into_iter()
                    .map(render_layer_bundle as fn(RenderLayer2d) -> RenderLayer2dBundle),
            )),
        }
    }
}

impl Default for LayerStack2dBundle {
    fn default() -> Self {
        Self::new([])
    }
}

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct LayerStack2dRootMarker;

#[derive(Bundle, Default)]
pub struct LayerStack2dRootBundle {
    marker: LayerStack2dRootMarker,
    transform: Transform,
    visibility: Visibility,
}

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub(in crate::primitives::layers) struct RenderLayer2dMarker;

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
}

#[derive(Component, Debug, Clone, Copy, Default, PartialEq)]
pub(in crate::primitives::layers) struct ParallaxLayer2d {
    pub(in crate::primitives::layers) speed: Vec2,
}

pub(crate) type LayerStackChildren2dBundle = SpawnRelatedBundle<
    ChildOf,
    SpawnIter<
        std::iter::Map<std::vec::IntoIter<RenderLayer2d>, fn(RenderLayer2d) -> RenderLayer2dBundle>,
    >,
>;

#[derive(Bundle)]
pub(crate) struct RenderLayer2dBundle {
    marker: RenderLayer2dMarker,
    parallax: ParallaxLayer2d,
    image: StaticImage2dBundle,
}

fn render_layer_bundle(layer: RenderLayer2d) -> RenderLayer2dBundle {
    let image = match layer.image {
        Some(image) => StaticImage2d::image(image, layer.size, layer.z),
        None => StaticImage2d::color(layer.color, layer.size, layer.z),
    };

    RenderLayer2dBundle {
        marker: RenderLayer2dMarker,
        parallax: ParallaxLayer2d {
            speed: layer.parallax_speed,
        },
        image: image.into_bundle(),
    }
}
