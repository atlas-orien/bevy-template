use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ExampleFrame2d {
    pub atlas_index: usize,
}

#[derive(Component, Debug, Clone)]
pub struct ExampleFrameAnimation2d {
    pub frames: Vec<ExampleFrame2d>,
    pub frames_per_second: f32,
    pub repeat: bool,
}

#[derive(Component, Debug, Clone, Copy, Default, PartialEq)]
pub struct ExampleFrameAnimationPlayback2d {
    pub elapsed_seconds: f32,
    pub current_frame: usize,
    pub playing: bool,
}
