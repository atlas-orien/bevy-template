use bevy::prelude::*;

#[derive(Component, Debug, Clone, Default, PartialEq)]
pub struct ExampleAtlasSprite2d {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    pub index: usize,
}

#[derive(Bundle)]
pub struct ExampleAtlasSprite2dBundle {
    pub atlas: ExampleAtlasSprite2d,
    pub sprite: Sprite,
}

impl ExampleAtlasSprite2dBundle {
    pub fn new(image: Handle<Image>, layout: Handle<TextureAtlasLayout>, index: usize) -> Self {
        Self {
            atlas: ExampleAtlasSprite2d {
                image: image.clone(),
                layout: layout.clone(),
                index,
            },
            sprite: Sprite::from_atlas_image(image, TextureAtlas { layout, index }),
        }
    }
}
