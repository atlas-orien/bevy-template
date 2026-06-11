use bevy::prelude::*;

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

impl AudioPlaybackSettings {
    pub fn looping() -> Self {
        Self {
            looping: true,
            ..Default::default()
        }
    }

    pub fn with_volume(mut self, volume: f32) -> Self {
        self.volume = volume;
        self
    }
}

impl From<AudioPlaybackSettings> for PlaybackSettings {
    fn from(settings: AudioPlaybackSettings) -> Self {
        let playback = if settings.looping {
            PlaybackSettings::LOOP
        } else {
            PlaybackSettings::DESPAWN
        };

        playback.with_volume(bevy::audio::Volume::Linear(settings.volume.max(0.0)))
    }
}
