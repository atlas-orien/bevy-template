use crate::bus::AudioBus;
use crate::playback::AudioPlaybackSettings;
use crate::source::{AudioSampleSource, ProceduralAudioSource};

#[derive(Debug, Clone, PartialEq)]
pub enum AudioSource {
    Sample(AudioSampleSource),
    Procedural(ProceduralAudioSource),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayAudioRequest {
    pub source: AudioSource,
    pub bus: AudioBus,
    pub settings: AudioPlaybackSettings,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct StopAudioRequest {
    pub id: u64,
}
