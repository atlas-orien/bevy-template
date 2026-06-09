#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AudioVolume(pub f32);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct AudioMute(pub bool);

impl Default for AudioVolume {
    fn default() -> Self {
        Self(1.0)
    }
}
