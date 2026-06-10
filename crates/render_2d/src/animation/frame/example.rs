use bevy::prelude::*;
use ecs::components::base::{Facing, MovementIntent};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ExampleFrame2d {
    pub atlas_index: usize,
}

#[derive(Component, Debug, Clone)]
pub struct ExampleFrameAnimation2d {
    pub frames: Vec<ExampleFrame2d>,
    pub frames_per_second: f32,
    pub repeat: bool,
}

#[derive(Component, Debug, Clone)]
pub struct ExampleGabeFrameClips2d {
    pub idle: ExampleFrameAnimation2d,
    pub run: ExampleFrameAnimation2d,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct ExampleFrameAnimationPlayback2d {
    pub elapsed_seconds: f32,
    pub current_frame: usize,
    pub playing: bool,
}

impl Default for ExampleFrameAnimationPlayback2d {
    fn default() -> Self {
        Self {
            elapsed_seconds: 0.0,
            current_frame: 0,
            playing: true,
        }
    }
}

type ExampleFrameAnimationQuery<'world, 'state> = Query<
    'world,
    'state,
    (
        Option<&'static ExampleFrameAnimation2d>,
        Option<&'static ExampleGabeFrameClips2d>,
        Option<&'static MovementIntent>,
        Option<&'static Facing>,
        &'static mut ExampleFrameAnimationPlayback2d,
        &'static mut Sprite,
    ),
>;

pub fn animate_example_frame_2d_system(time: Res<Time>, mut sprites: ExampleFrameAnimationQuery) {
    for (animation, gabe_clips, movement_intent, facing, mut playback, mut sprite) in &mut sprites {
        if let Some(facing) = facing {
            sprite.flip_x = *facing == Facing::Left;
        }

        let selected_animation = match (
            gabe_clips,
            movement_intent.is_some_and(|intent| intent.is_moving()),
        ) {
            (Some(clips), true) => Some(&clips.run),
            (Some(clips), false) => Some(&clips.idle),
            (None, _) => animation,
        };

        let Some(animation) = selected_animation else {
            continue;
        };

        if !playback.playing || animation.frames.is_empty() || animation.frames_per_second <= 0.0 {
            continue;
        }
        if playback.current_frame >= animation.frames.len() {
            playback.current_frame = 0;
            playback.elapsed_seconds = 0.0;
        }

        playback.elapsed_seconds += time.delta_secs();
        let seconds_per_frame = 1.0 / animation.frames_per_second;

        while playback.elapsed_seconds >= seconds_per_frame {
            playback.elapsed_seconds -= seconds_per_frame;
            playback.current_frame += 1;

            if playback.current_frame >= animation.frames.len() {
                if animation.repeat {
                    playback.current_frame = 0;
                } else {
                    playback.current_frame = animation.frames.len() - 1;
                    playback.playing = false;
                }
            }
        }

        let atlas_index = animation.frames[playback.current_frame].atlas_index;
        if let Some(texture_atlas) = &mut sprite.texture_atlas {
            texture_atlas.index = atlas_index;
        }
    }
}

pub fn example_gabe_frame_clips_2d() -> ExampleGabeFrameClips2d {
    ExampleGabeFrameClips2d {
        idle: example_frame_animation_2d([0, 1, 2], 8.0),
        run: example_frame_animation_2d([3, 4, 5, 6], 8.0),
    }
}

fn example_frame_animation_2d(
    frames: impl IntoIterator<Item = usize>,
    frames_per_second: f32,
) -> ExampleFrameAnimation2d {
    ExampleFrameAnimation2d {
        frames: frames
            .into_iter()
            .map(|atlas_index| ExampleFrame2d { atlas_index })
            .collect(),
        frames_per_second,
        repeat: true,
    }
}
