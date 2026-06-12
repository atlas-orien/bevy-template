//! Audio foundation types, playback requests, and Bevy audio bridge systems.

pub mod bus;
pub mod playback;
pub mod request;
pub mod source;
pub mod spatial;
pub mod volume;

pub use error::Result;

use bevy::prelude::*;

use crate::request::{
    AudioPlaybackBus, AudioPlaybackId, AudioSource, NextAudioPlaybackId, PlayAudioRequest,
    StopAudioRequest,
};

pub struct AudioFoundationPlugin;

impl Plugin for AudioFoundationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NextAudioPlaybackId>()
            .add_message::<PlayAudioRequest>()
            .add_message::<StopAudioRequest>()
            .add_systems(Update, (play_sample_audio, stop_audio));
    }
}

fn play_sample_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_id: ResMut<NextAudioPlaybackId>,
    mut requests: MessageReader<PlayAudioRequest>,
) {
    for request in requests.read() {
        let AudioSource::Sample(source) = &request.source else {
            continue;
        };

        let id = next_id.allocate();
        let audio = asset_server.load(source.path.clone());

        commands.spawn((
            AudioPlayer::new(audio),
            PlaybackSettings::from(request.settings),
            AudioPlaybackId(id.0),
            AudioPlaybackBus(request.bus),
        ));
    }
}

fn stop_audio(
    mut commands: Commands,
    mut requests: MessageReader<StopAudioRequest>,
    playing: Query<(Entity, &AudioPlaybackId)>,
) {
    for request in requests.read() {
        for (entity, playback_id) in &playing {
            if *playback_id == request.id {
                commands.entity(entity).despawn();
            }
        }
    }
}
