//! 把 demo 生命周期与移动状态桥接成音频播放请求。

use audio::playback::AudioPlaybackSettings;
use audio::request::PlayAudioRequest;
use bevy::prelude::*;
use ecs::components::base::{AudioClips, MovementIntent};
use ecs::components::characters::DemoPlayerControlled;
use ecs::events::demo_sensor::DemoSensorTriggeredEvent;
use ecs::events::demo_session::DemoSessionStartedEvent;

const DEMO_BGM_AUDIO: &str = "audio/demo_bgm.ogg";
const DEMO_BGM_VOLUME: f32 = 0.35;
const DEMO_FOOTSTEP_INTERVAL_SECONDS: f32 = 0.28;

pub fn demo_bgm_audio_system(
    mut events: MessageReader<DemoSessionStartedEvent>,
    mut audio_requests: MessageWriter<PlayAudioRequest>,
) {
    for _ in events.read() {
        audio_requests.write(
            PlayAudioRequest::sample(DEMO_BGM_AUDIO)
                .with_settings(AudioPlaybackSettings::looping().with_volume(DEMO_BGM_VOLUME)),
        );
    }
}

pub fn demo_sensor_audio_system(
    mut events: MessageReader<DemoSensorTriggeredEvent>,
    audio_clips: Query<&AudioClips>,
    mut audio_requests: MessageWriter<PlayAudioRequest>,
) {
    for event in events.read() {
        let Ok(clips) = audio_clips.get(event.sensor) else {
            continue;
        };
        let Some(clip) = &clips.interact else {
            continue;
        };

        audio_requests.write(PlayAudioRequest::sample(clip.path()));
    }
}

pub fn demo_footstep_audio_system(
    time: Res<Time>,
    mut timer: Local<Option<Timer>>,
    players: Query<(&MovementIntent, &AudioClips), With<DemoPlayerControlled>>,
    mut audio_requests: MessageWriter<PlayAudioRequest>,
) {
    let timer = timer.get_or_insert_with(|| {
        Timer::from_seconds(DEMO_FOOTSTEP_INTERVAL_SECONDS, TimerMode::Repeating)
    });
    timer.tick(time.delta());

    if !timer.just_finished() {
        return;
    }

    for (movement, clips) in &players {
        if !movement.is_moving() {
            continue;
        }
        let Some(clip) = &clips.interact else {
            continue;
        };

        audio_requests.write(PlayAudioRequest::sample(clip.path()));
    }
}
