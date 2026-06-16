//! 通用图片表现 primitive。

use bevy::prelude::*;

pub struct StaticImage2d {
    color: Color,
    image: Option<Handle<Image>>,
    size: Vec2,
    z: f32,
}

impl StaticImage2d {
    pub fn color(color: Color, size: Vec2, z: f32) -> Self {
        Self {
            color,
            image: None,
            size,
            z,
        }
    }

    pub fn image(image: Handle<Image>, size: Vec2, z: f32) -> Self {
        Self {
            color: Color::WHITE,
            image: Some(image),
            size,
            z,
        }
    }

    pub fn into_bundle(self) -> StaticImage2dBundle {
        let mut sprite = Sprite::from_color(self.color, self.size);
        if let Some(image) = self.image {
            sprite.image = image;
        }

        StaticImage2dBundle {
            sprite,
            transform: Transform::from_xyz(0.0, 0.0, self.z),
            visibility: Visibility::default(),
        }
    }
}

#[derive(Bundle)]
pub struct StaticImage2dBundle {
    sprite: Sprite,
    transform: Transform,
    visibility: Visibility,
}
