//! Demo 玩家 sprite 表现。

use bevy::prelude::*;

use crate::animation::frame::{
    FrameAnimation2d, FrameAnimationFacingFlip2d, FrameAnimationHandle2d, FrameAnimationManifest2d,
    FrameAnimationMovementClips2d,
};

const DEMO_IDLE_CLIP: &str = "idle";
const DEMO_WALK_CLIP: &str = "walk";
const DEMO_PLAYER_SPRITE_SIZE: Vec2 = Vec2::new(48.0, 48.0);
const DEMO_PLAYER_SPRITE_TRANSLATION: Vec3 = Vec3::new(0.0, 18.0, 4.0);

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub(super) struct DemoPlayerSprite2dMarker;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub(super) struct DemoPlayerSpriteAtlasReady2d;

type PendingDemoPlayerSpriteAtlasQuery<'world, 'state> = Query<
    'world,
    'state,
    (Entity, &'static FrameAnimationHandle2d, &'static mut Sprite),
    (
        With<DemoPlayerSprite2dMarker>,
        Without<DemoPlayerSpriteAtlasReady2d>,
    ),
>;

#[derive(Bundle)]
pub struct DemoPlayerSprite2d {
    marker: DemoPlayerSprite2dMarker,
    frame_manifest: FrameAnimationHandle2d,
    movement_clips: FrameAnimationMovementClips2d,
    facing_flip: FrameAnimationFacingFlip2d,
    animation: FrameAnimation2d,
    sprite: Sprite,
    transform: Transform,
}

impl DemoPlayerSprite2d {
    pub fn new(frame_manifest: Handle<FrameAnimationManifest2d>) -> Self {
        Self {
            marker: DemoPlayerSprite2dMarker,
            frame_manifest: FrameAnimationHandle2d(frame_manifest),
            movement_clips: FrameAnimationMovementClips2d::new(DEMO_IDLE_CLIP, DEMO_WALK_CLIP),
            facing_flip: FrameAnimationFacingFlip2d,
            animation: FrameAnimation2d::new(DEMO_IDLE_CLIP),
            sprite: Sprite {
                custom_size: Some(DEMO_PLAYER_SPRITE_SIZE),
                ..default()
            },
            transform: Transform::from_translation(DEMO_PLAYER_SPRITE_TRANSLATION),
        }
    }
}

pub(super) fn prepare_demo_player_sprite_atlas_system(
    mut commands: Commands,
    frame_manifests: Res<Assets<FrameAnimationManifest2d>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut sprites: PendingDemoPlayerSpriteAtlasQuery,
) {
    for (entity, manifest_handle, mut sprite) in &mut sprites {
        let Some(manifest) = frame_manifests.get(&manifest_handle.0) else {
            continue;
        };

        sprite.image = manifest.image.clone();
        sprite.texture_atlas = Some(TextureAtlas {
            layout: atlas_layouts.add(manifest.atlas_layout()),
            index: 0,
        });
        commands.entity(entity).insert(DemoPlayerSpriteAtlasReady2d);
    }
}
