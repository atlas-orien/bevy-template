use audio::request::PlayAudioRequest;
use bevy::prelude::*;
use ecs::components::base::{AudioClips, MovementIntent};
use ecs::components::characters::DemoPlayerControlled;
use ecs::events::demo_sensor::DemoSensorTriggeredEvent;

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
    let timer = timer.get_or_insert_with(|| Timer::from_seconds(0.28, TimerMode::Repeating));
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
