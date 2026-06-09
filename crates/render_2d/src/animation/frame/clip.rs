use super::SpriteAnimationFrame2d;

#[derive(Debug, Clone)]
pub struct SpriteAnimationClip2d {
    pub frames: Vec<SpriteAnimationFrame2d>,
    pub frames_per_second: f32,
    pub repeat: bool,
}

impl SpriteAnimationClip2d {
    pub fn new(
        frames: impl Into<Vec<SpriteAnimationFrame2d>>,
        frames_per_second: f32,
        repeat: bool,
    ) -> Self {
        Self {
            frames: frames.into(),
            frames_per_second,
            repeat,
        }
    }
}
