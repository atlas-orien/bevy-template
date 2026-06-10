use bevy::prelude::*;

use crate::animation::frame::{
    ExampleFrameAnimationPlayback2d, ExampleGabeFrameClips2d, example_gabe_frame_clips_2d,
};

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct ExampleCharacter2d;

#[derive(Bundle)]
pub struct ExampleCharacter2dBundle {
    pub marker: ExampleCharacter2d,
    pub sprite: Sprite,
}

impl ExampleCharacter2dBundle {
    pub fn new(color: Color, size: Vec2) -> Self {
        Self {
            marker: ExampleCharacter2d,
            sprite: Sprite {
                color,
                custom_size: Some(size),
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct ExampleGabeCharacter2dBundle {
    pub marker: ExampleCharacter2d,
    pub sprite: Sprite,
    pub frame_clips: ExampleGabeFrameClips2d,
    pub frame_playback: ExampleFrameAnimationPlayback2d,
}

impl ExampleGabeCharacter2dBundle {
    pub fn new(image: Handle<Image>, layout: Handle<TextureAtlasLayout>) -> Self {
        Self {
            marker: ExampleCharacter2d,
            sprite: Sprite {
                image,
                texture_atlas: Some(TextureAtlas { layout, index: 0 }),
                ..default()
            },
            frame_clips: example_gabe_frame_clips_2d(),
            frame_playback: ExampleFrameAnimationPlayback2d::default(),
        }
    }
}
