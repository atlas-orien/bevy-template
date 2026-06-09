use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, PartialEq)]
pub struct SpriteAnimationPlayback2d {
    pub elapsed_seconds: f32,
    pub current_frame: usize,
    pub playing: bool,
}
