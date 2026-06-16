//! 通用逐帧动画推进系统。

use bevy::prelude::*;

use super::{FrameAnimation2d, FrameAnimationHandle2d, FrameAnimationManifest2d};

pub struct FrameAnimationBasePlugin;

impl Plugin for FrameAnimationBasePlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<FrameAnimationManifest2d>()
            .init_asset_loader::<super::FrameAnimationManifestLoader2d>()
            .add_systems(Update, frame_animation_system);
    }
}

fn frame_animation_system(
    time: Res<Time>,
    frame_manifests: Res<Assets<FrameAnimationManifest2d>>,
    mut sprites: Query<(&FrameAnimationHandle2d, &mut FrameAnimation2d, &mut Sprite)>,
) {
    for (manifest_handle, mut animation, mut sprite) in &mut sprites {
        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            continue;
        };
        let Some(manifest) = frame_manifests.get(&manifest_handle.0) else {
            continue;
        };
        let Some(clip) = manifest.clip(&animation.clip) else {
            continue;
        };
        if !clip.repeat && animation.current_frame() + 1 >= clip.frames.len() {
            let Some(&frame) = clip.frames.get(animation.current_frame()) else {
                continue;
            };
            atlas.index = frame;
            continue;
        }

        animation.tick(time.delta_secs(), 1.0 / clip.fps, clip.frames.len());

        let Some(&frame) = clip.frames.get(animation.current_frame()) else {
            continue;
        };
        atlas.index = frame;
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use crate::animation::frame::FrameAnimationClip2d;

    const IDLE_CLIP: &str = "idle";
    const WALK_CLIP: &str = "walk";
    const FRAME_SECONDS: f32 = 0.12;

    fn animation_app(delta_seconds: f32) -> (App, Handle<FrameAnimationManifest2d>) {
        let mut app = App::new();
        let mut time = Time::<()>::default();
        time.advance_by(Duration::from_secs_f32(delta_seconds));
        let mut frame_manifests = Assets::<FrameAnimationManifest2d>::default();
        let manifest_handle = frame_manifests.add(FrameAnimationManifest2d {
            image: Handle::default(),
            frame_size: UVec2::new(24, 24),
            columns: 7,
            rows: 1,
            clips: [
                (
                    IDLE_CLIP.to_string(),
                    FrameAnimationClip2d {
                        frames: vec![0],
                        fps: 1.0 / FRAME_SECONDS,
                        repeat: true,
                    },
                ),
                (
                    WALK_CLIP.to_string(),
                    FrameAnimationClip2d {
                        frames: vec![1, 2, 3, 4, 5, 6],
                        fps: 1.0 / FRAME_SECONDS,
                        repeat: true,
                    },
                ),
            ]
            .into(),
        });
        app.insert_resource(time)
            .insert_resource(frame_manifests)
            .add_systems(Update, frame_animation_system);
        (app, manifest_handle)
    }

    fn animated_sprite(
        index: usize,
        animation: FrameAnimation2d,
        manifest: Handle<FrameAnimationManifest2d>,
    ) -> impl Bundle {
        (
            FrameAnimationHandle2d(manifest),
            animation,
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
        let (mut app, manifest) = animation_app(FRAME_SECONDS);
        let entity = app
            .world_mut()
            .spawn(animated_sprite(
                4,
                FrameAnimation2d::new(IDLE_CLIP),
                manifest,
            ))
            .id();

        app.update();

        let sprite = app.world().get::<Sprite>(entity).unwrap();
        assert_eq!(sprite.texture_atlas.as_ref().unwrap().index, 0);
    }

    #[test]
    fn multi_frame_animation_advances_after_frame_seconds() {
        let (mut app, manifest) = animation_app(FRAME_SECONDS);
        let animation = FrameAnimation2d::new(WALK_CLIP);
        let entity = app
            .world_mut()
            .spawn(animated_sprite(1, animation, manifest))
            .id();

        app.update();

        let sprite = app.world().get::<Sprite>(entity).unwrap();
        assert_eq!(sprite.texture_atlas.as_ref().unwrap().index, 2);
        let animation = app.world().get::<FrameAnimation2d>(entity).unwrap();
        assert_eq!(animation.current_frame(), 1);
    }

    #[test]
    fn multi_frame_animation_wraps_from_last_to_first() {
        let (mut app, manifest) = animation_app(FRAME_SECONDS);
        let mut animation = FrameAnimation2d::new(WALK_CLIP);
        animation.set_current_frame_for_test(5);
        let entity = app
            .world_mut()
            .spawn(animated_sprite(6, animation, manifest))
            .id();

        app.update();

        let sprite = app.world().get::<Sprite>(entity).unwrap();
        assert_eq!(sprite.texture_atlas.as_ref().unwrap().index, 1);
        let animation = app.world().get::<FrameAnimation2d>(entity).unwrap();
        assert_eq!(animation.current_frame(), 0);
    }

    #[test]
    fn set_clip_resets_elapsed_only_when_clip_changes() {
        let mut animation = FrameAnimation2d::new(IDLE_CLIP);
        animation.set_clip(WALK_CLIP);
        assert!(!animation.tick(FRAME_SECONDS / 2.0, FRAME_SECONDS, 2));

        animation.set_clip(WALK_CLIP);
        assert!(animation.elapsed_seconds_for_test() > 0.0);

        animation.set_clip(IDLE_CLIP);
        assert_eq!(animation.elapsed_seconds_for_test(), 0.0);
        assert!(!animation.tick(FRAME_SECONDS / 2.0, FRAME_SECONDS, 1));
    }
}
