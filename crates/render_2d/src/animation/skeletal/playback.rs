use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, PartialEq)]
pub struct SkeletalAnimationPlayback2d {
    pub elapsed_seconds: f32,
    pub playing: bool,
}
