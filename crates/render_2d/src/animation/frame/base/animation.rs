//! 通用逐帧动画播放状态。

use bevy::prelude::*;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct FrameAnimation2d {
    pub clip: String,
    current_frame: usize,
    elapsed_seconds: f32,
}

impl FrameAnimation2d {
    pub fn new(clip: impl Into<String>) -> Self {
        Self {
            clip: clip.into(),
            current_frame: 0,
            elapsed_seconds: 0.0,
        }
    }

    pub fn set_clip(&mut self, clip: impl Into<String>) {
        let clip = clip.into();
        if self.clip == clip {
            return;
        }

        self.clip = clip;
        self.current_frame = 0;
        self.elapsed_seconds = 0.0;
    }

    pub fn current_frame(&self) -> usize {
        self.current_frame
    }

    pub fn set_current_frame_for_test(&mut self, current_frame: usize) {
        self.current_frame = current_frame;
    }

    pub fn elapsed_seconds_for_test(&self) -> f32 {
        self.elapsed_seconds
    }

    pub fn tick(&mut self, delta_seconds: f32, frame_seconds: f32, frame_count: usize) -> bool {
        if frame_count <= 1 {
            return false;
        }

        self.elapsed_seconds += delta_seconds;
        if self.elapsed_seconds < frame_seconds {
            return false;
        }

        self.elapsed_seconds = 0.0;
        self.current_frame = (self.current_frame + 1) % frame_count;
        true
    }
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct FrameAnimationMovementClips2d {
    pub idle: String,
    pub moving: String,
}

impl FrameAnimationMovementClips2d {
    pub fn new(idle: impl Into<String>, moving: impl Into<String>) -> Self {
        Self {
            idle: idle.into(),
            moving: moving.into(),
        }
    }
}

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct FrameAnimationFacingFlip2d;
