use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct AudioClip(pub String);

impl AudioClip {
    pub fn new(path: impl Into<String>) -> Self {
        Self(path.into())
    }

    pub fn path(&self) -> &str {
        &self.0
    }
}

#[derive(Component, Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct AudioClips {
    pub spawn: Option<AudioClip>,
    pub despawn: Option<AudioClip>,
    pub interact: Option<AudioClip>,
    pub hit: Option<AudioClip>,
}

impl AudioClips {
    pub fn with_spawn(mut self, path: impl Into<String>) -> Self {
        self.spawn = Some(AudioClip::new(path));
        self
    }

    pub fn with_despawn(mut self, path: impl Into<String>) -> Self {
        self.despawn = Some(AudioClip::new(path));
        self
    }

    pub fn with_interact(mut self, path: impl Into<String>) -> Self {
        self.interact = Some(AudioClip::new(path));
        self
    }

    pub fn with_hit(mut self, path: impl Into<String>) -> Self {
        self.hit = Some(AudioClip::new(path));
        self
    }
}
