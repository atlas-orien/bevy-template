use crate::bus::AudioBus;
use crate::playback::AudioPlaybackSettings;
use crate::source::{AudioSampleSource, ProceduralAudioSource};

use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum AudioSource {
    Sample(AudioSampleSource),
    Procedural(ProceduralAudioSource),
}

#[derive(Message, Debug, Clone, PartialEq)]
pub struct PlayAudioRequest {
    pub source: AudioSource,
    pub bus: AudioBus,
    pub settings: AudioPlaybackSettings,
}

#[derive(Message, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct StopAudioRequest {
    pub id: AudioPlaybackId,
}

impl PlayAudioRequest {
    pub fn sample(path: impl Into<String>) -> Self {
        Self {
            source: AudioSource::Sample(AudioSampleSource::new(path)),
            bus: AudioBus::default(),
            settings: AudioPlaybackSettings::default(),
        }
    }

    pub fn with_bus(mut self, bus: AudioBus) -> Self {
        self.bus = bus;
        self
    }

    pub fn with_settings(mut self, settings: AudioPlaybackSettings) -> Self {
        self.settings = settings;
        self
    }
}

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct AudioPlaybackId(pub u64);

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct AudioPlaybackBus(pub AudioBus);

#[derive(Resource, Debug, Default)]
pub struct NextAudioPlaybackId(u64);

impl NextAudioPlaybackId {
    pub fn allocate(&mut self) -> AudioPlaybackId {
        let id = AudioPlaybackId(self.0);
        self.0 = self.0.saturating_add(1);
        id
    }
}
