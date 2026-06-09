#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AudioPlaybackSettings {
    pub volume: f32,
    pub looping: bool,
}

impl Default for AudioPlaybackSettings {
    fn default() -> Self {
        Self {
            volume: 1.0,
            looping: false,
        }
    }
}
