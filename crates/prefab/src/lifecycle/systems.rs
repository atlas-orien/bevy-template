use audio::request::PlayAudioRequest;
use bevy::prelude::*;
use ecs::components::base::AudioClips;
use ecs::events::lifecycle::{DiedEvent, SpawnedEvent};

use ecs::components::world::gameplay::GameplaySessionEntityMarker;

pub type GameplaySessionEntities<'world, 'state> =
    Query<'world, 'state, Entity, With<GameplaySessionEntityMarker>>;

pub fn despawn_gameplay_prefabs(
    commands: &mut Commands,
    entities: &GameplaySessionEntities<'_, '_>,
) {
    for entity in entities {
        commands.entity(entity).try_despawn();
    }
}

pub fn play_spawn_audio_system(
    mut events: MessageReader<SpawnedEvent>,
    audio_clips: Query<&AudioClips>,
    mut audio_requests: MessageWriter<PlayAudioRequest>,
) {
    for event in events.read() {
        let Ok(clips) = audio_clips.get(event.entity) else {
            continue;
        };
        let Some(clip) = &clips.spawn else {
            continue;
        };

        audio_requests.write(PlayAudioRequest::sample(clip.path()));
    }
}

pub fn play_despawn_audio_system(
    mut events: MessageReader<DiedEvent>,
    audio_clips: Query<&AudioClips>,
    mut audio_requests: MessageWriter<PlayAudioRequest>,
) {
    for event in events.read() {
        let Ok(clips) = audio_clips.get(event.entity) else {
            continue;
        };
        let Some(clip) = &clips.despawn else {
            continue;
        };

        audio_requests.write(PlayAudioRequest::sample(clip.path()));
    }
}
