use bevy::prelude::*;

use super::layers::{LayeredBackground2dBundle, LayeredBackgroundLayers2d};

#[derive(Default)]
pub struct LayeredBackground2d {
    pub bundle: LayeredBackground2dBundle,
    layers: LayeredBackgroundLayers2d,
}

impl LayeredBackground2d {
    pub fn new(layers: impl IntoIterator<Item = BackgroundLayer2d>) -> Self {
        Self {
            bundle: LayeredBackground2dBundle::default(),
            layers: LayeredBackgroundLayers2d::new(layers),
        }
    }

    pub fn into_bundle(self) -> impl Bundle {
        (self.bundle, self.layers.into_children())
    }
}

pub struct BackgroundLayer2d {
    color: Color,
    image: Option<Handle<Image>>,
    size: Vec2,
    z: f32,
    parallax_speed: Vec2,
}

impl BackgroundLayer2d {
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

    pub(in crate::background::layered) fn into_bundle(self) -> impl Bundle {
        let mut sprite = Sprite::from_color(self.color, self.size);
        if let Some(image) = self.image {
            sprite.image = image;
        }

        (
            BackgroundLayer2dMarker,
            ParallaxBackgroundLayer2d {
                speed: self.parallax_speed,
            },
            sprite,
            Transform::from_xyz(0.0, 0.0, self.z),
        )
    }
}

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub(in crate::background) struct BackgroundLayer2dMarker;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct LayeredBackgroundRoot2d;

#[derive(Component, Debug, Clone, Copy, Default, PartialEq)]
pub(in crate::background) struct ParallaxBackgroundLayer2d {
    pub(in crate::background) speed: Vec2,
}
