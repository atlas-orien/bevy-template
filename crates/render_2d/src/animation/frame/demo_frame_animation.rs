//! Demo 角色的逐帧动画状态与推进系统。

use bevy::prelude::*;
use ecs::components::base::{Facing, MovementIntent};

use super::demo_frame_manifest::{DemoFrameManifest2d, DemoFrameManifestHandle2d};

pub const DEMO_IDLE_CLIP: &str = "idle";
pub const DEMO_WALK_CLIP: &str = "walk";

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct DemoFrameAnimation2d {
    pub clip: &'static str,
    current_frame: usize,
    elapsed_seconds: f32,
}

impl DemoFrameAnimation2d {
    pub fn idle() -> Self {
        Self {
            clip: DEMO_IDLE_CLIP,
            current_frame: 0,
            elapsed_seconds: 0.0,
        }
    }

    pub fn set_clip(&mut self, clip: &'static str) {
        if self.clip == clip {
            return;
        }

        self.clip = clip;
        self.current_frame = 0;
        self.elapsed_seconds = 0.0;
    }

    pub fn current_frame(&self) -> usize {
        self.current_frame
    }

    pub fn tick(&mut self, delta_seconds: f32, frame_seconds: f32, frame_count: usize) -> bool {
        if frame_count <= 1 {
            return false;
        }

        self.elapsed_seconds += delta_seconds;
        if self.elapsed_seconds < frame_seconds {
            return false;
        }

        self.elapsed_seconds = 0.0;
        self.current_frame = (self.current_frame + 1) % frame_count;
        true
    }
}

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoPlayerAnimation2d;

pub fn demo_player_animation_state_system(
    parents: Query<(&MovementIntent, Option<&Facing>)>,
    mut sprites: Query<
        (&ChildOf, &mut DemoFrameAnimation2d, &mut Sprite),
        With<DemoPlayerAnimation2d>,
    >,
) {
    for (parent, mut animation, mut sprite) in &mut sprites {
        let Ok((movement, facing)) = parents.get(parent.parent()) else {
            continue;
        };

        animation.set_clip(if movement.is_moving() {
            DEMO_WALK_CLIP
        } else {
            DEMO_IDLE_CLIP
        });

        if let Some(facing) = facing {
            sprite.flip_x = *facing == Facing::Left;
        }
    }
}

pub fn demo_frame_animation_system(
    time: Res<Time>,
    frame_manifests: Res<Assets<DemoFrameManifest2d>>,
    mut sprites: Query<
        (
            &DemoFrameManifestHandle2d,
            &mut DemoFrameAnimation2d,
            &mut Sprite,
        ),
        With<DemoPlayerAnimation2d>,
    >,
) {
    for (manifest_handle, mut animation, mut sprite) in &mut sprites {
        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            continue;
        };
        let Some(manifest) = frame_manifests.get(&manifest_handle.0) else {
            continue;
        };
        let Some(clip) = manifest.clip(animation.clip) else {
            continue;
        };
        if !clip.repeat && animation.current_frame + 1 >= clip.frames.len() {
            let Some(&frame) = clip.frames.get(animation.current_frame) else {
                continue;
            };
            atlas.index = frame;
            continue;
        }

        animation.tick(time.delta_secs(), 1.0 / clip.fps, clip.frames.len());

        let Some(&frame) = clip.frames.get(animation.current_frame) else {
            continue;
        };
        atlas.index = frame;
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use crate::animation::frame::demo_frame_manifest::DemoFrameClip2d;

    const DEMO_FRAME_SECONDS: f32 = 0.12;

    fn animation_app(delta_seconds: f32) -> (App, Handle<DemoFrameManifest2d>) {
        let mut app = App::new();
        let mut time = Time::<()>::default();
        time.advance_by(Duration::from_secs_f32(delta_seconds));
        let mut frame_manifests = Assets::<DemoFrameManifest2d>::default();
        let manifest_handle = frame_manifests.add(DemoFrameManifest2d {
            image: Handle::default(),
            frame_size: UVec2::new(24, 24),
            columns: 7,
            rows: 1,
            clips: [
                (
                    DEMO_IDLE_CLIP.to_string(),
                    DemoFrameClip2d {
                        frames: vec![0],
                        fps: 1.0 / DEMO_FRAME_SECONDS,
                        repeat: true,
                    },
                ),
                (
                    DEMO_WALK_CLIP.to_string(),
                    DemoFrameClip2d {
                        frames: vec![1, 2, 3, 4, 5, 6],
                        fps: 1.0 / DEMO_FRAME_SECONDS,
                        repeat: true,
                    },
                ),
            ]
            .into(),
        });
        app.insert_resource(time)
            .insert_resource(frame_manifests)
            .add_systems(Update, demo_frame_animation_system);
        (app, manifest_handle)
    }

    fn animated_sprite(
        index: usize,
        animation: DemoFrameAnimation2d,
        manifest: Handle<DemoFrameManifest2d>,
    ) -> impl Bundle {
        (
            DemoFrameManifestHandle2d(manifest),
            animation,
            DemoPlayerAnimation2d,
            Sprite {
                texture_atlas: Some(TextureAtlas {
                    layout: Handle::default(),
                    index,
                }),
                ..default()
            },
        )
    }

    #[test]
    fn single_frame_animation_stays_on_frame() {
        let (mut app, manifest) = animation_app(DEMO_FRAME_SECONDS);
        let entity = app
            .world_mut()
            .spawn(animated_sprite(4, DemoFrameAnimation2d::idle(), manifest))
            .id();

        app.update();

        let sprite = app.world().get::<Sprite>(entity).unwrap();
        assert_eq!(sprite.texture_atlas.as_ref().unwrap().index, 0);
    }

    #[test]
    fn multi_frame_animation_advances_after_frame_seconds() {
        let (mut app, manifest) = animation_app(DEMO_FRAME_SECONDS);
        let mut animation = DemoFrameAnimation2d::idle();
        animation.set_clip(DEMO_WALK_CLIP);
        let entity = app
            .world_mut()
            .spawn(animated_sprite(1, animation, manifest))
            .id();

        app.update();

        let sprite = app.world().get::<Sprite>(entity).unwrap();
        assert_eq!(sprite.texture_atlas.as_ref().unwrap().index, 2);
        let animation = app.world().get::<DemoFrameAnimation2d>(entity).unwrap();
        assert_eq!(animation.current_frame(), 1);
    }

    #[test]
    fn multi_frame_animation_wraps_from_last_to_first() {
        let (mut app, manifest) = animation_app(DEMO_FRAME_SECONDS);
        let mut animation = DemoFrameAnimation2d::idle();
        animation.set_clip(DEMO_WALK_CLIP);
        animation.current_frame = 5;
        let entity = app
            .world_mut()
            .spawn(animated_sprite(6, animation, manifest))
            .id();

        app.update();

        let sprite = app.world().get::<Sprite>(entity).unwrap();
        assert_eq!(sprite.texture_atlas.as_ref().unwrap().index, 1);
        let animation = app.world().get::<DemoFrameAnimation2d>(entity).unwrap();
        assert_eq!(animation.current_frame(), 0);
    }

    #[test]
    fn set_clip_resets_elapsed_only_when_clip_changes() {
        let mut animation = DemoFrameAnimation2d::idle();
        animation.set_clip(DEMO_WALK_CLIP);
        assert!(!animation.tick(DEMO_FRAME_SECONDS / 2.0, DEMO_FRAME_SECONDS, 2));

        animation.set_clip(DEMO_WALK_CLIP);
        assert!(animation.elapsed_seconds > 0.0);

        animation.set_clip(DEMO_IDLE_CLIP);
        assert_eq!(animation.elapsed_seconds, 0.0);
        assert!(!animation.tick(DEMO_FRAME_SECONDS / 2.0, DEMO_FRAME_SECONDS, 1));
    }
}
