mod plugin;

use bevy::prelude::*;

pub use plugin::Text2dContentPlugin;

pub struct WorldText2d {
    text: String,
    font: Handle<Font>,
    font_size: f32,
    color: Color,
    translation: Vec3,
}

impl WorldText2d {
    pub fn new(
        text: impl Into<String>,
        font: Handle<Font>,
        font_size: f32,
        color: Color,
        translation: Vec3,
    ) -> Self {
        Self {
            text: text.into(),
            font,
            font_size,
            color,
            translation,
        }
    }

    pub fn into_bundle(self) -> WorldText2dBundle {
        WorldText2dBundle {
            text: Text2d::new(self.text),
            font: TextFont {
                font: self.font,
                font_size: self.font_size,
                ..default()
            },
            color: TextColor(self.color),
            transform: Transform::from_translation(self.translation),
            visibility: Visibility::default(),
        }
    }
}

#[derive(Bundle)]
pub struct WorldText2dBundle {
    text: Text2d,
    font: TextFont,
    color: TextColor,
    transform: Transform,
    visibility: Visibility,
}
