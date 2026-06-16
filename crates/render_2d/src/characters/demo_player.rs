//! Demo 角色的 sprite 表现 bundle。

use bevy::prelude::*;

use crate::animation::frame::{DemoFrameAnimation2d, DemoPlayerAnimation2d};
use crate::animation::frame::{DemoFrameManifest2d, DemoFrameManifestHandle2d};

const DEMO_PLAYER_SPRITE_SIZE: Vec2 = Vec2::new(48.0, 48.0);
const DEMO_PLAYER_SPRITE_TRANSLATION: Vec3 = Vec3::new(0.0, 18.0, 4.0);
const DEMO_NPC_SPRITE_COLOR: Color = Color::srgb(0.65, 0.42, 0.95);
const DEMO_NPC_SPRITE_SIZE: Vec2 = Vec2::new(30.0, 38.0);
const DEMO_NPC_SPRITE_TRANSLATION: Vec3 = Vec3::new(0.0, 18.0, 4.0);

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoPlayerSprite2d;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoPlayerSpriteAtlasReady2d;

type PendingDemoPlayerSpriteAtlasQuery<'world, 'state> = Query<
    'world,
    'state,
    (
        Entity,
        &'static DemoFrameManifestHandle2d,
        &'static mut Sprite,
    ),
    (
        With<DemoPlayerSprite2d>,
        Without<DemoPlayerSpriteAtlasReady2d>,
    ),
>;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoNpcSprite2d;

#[derive(Bundle)]
pub struct DemoPlayerSprite2dBundle {
    pub marker: DemoPlayerSprite2d,
    pub frame_manifest: DemoFrameManifestHandle2d,
    pub animation_marker: DemoPlayerAnimation2d,
    pub animation: DemoFrameAnimation2d,
    pub sprite: Sprite,
    pub transform: Transform,
}

impl DemoPlayerSprite2dBundle {
    pub fn new(frame_manifest: Handle<DemoFrameManifest2d>) -> Self {
        Self {
            marker: DemoPlayerSprite2d,
            frame_manifest: DemoFrameManifestHandle2d(frame_manifest),
            animation_marker: DemoPlayerAnimation2d,
            animation: DemoFrameAnimation2d::idle(),
            sprite: Sprite {
                custom_size: Some(DEMO_PLAYER_SPRITE_SIZE),
                ..default()
            },
            transform: Transform::from_translation(DEMO_PLAYER_SPRITE_TRANSLATION),
        }
    }
}

pub fn prepare_demo_player_sprite_atlas_system(
    mut commands: Commands,
    frame_manifests: Res<Assets<DemoFrameManifest2d>>,
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

#[derive(Bundle)]
pub struct DemoNpcSprite2dBundle {
    pub marker: DemoNpcSprite2d,
    pub sprite: Sprite,
    pub transform: Transform,
}

impl Default for DemoNpcSprite2dBundle {
    fn default() -> Self {
        Self {
            marker: DemoNpcSprite2d,
            sprite: Sprite {
                color: DEMO_NPC_SPRITE_COLOR,
                custom_size: Some(DEMO_NPC_SPRITE_SIZE),
                ..default()
            },
            transform: Transform::from_translation(DEMO_NPC_SPRITE_TRANSLATION),
        }
    }
}
