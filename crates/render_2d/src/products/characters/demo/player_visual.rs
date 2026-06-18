//! Demo 玩家完整 2D 表现。

use bevy::ecs::hierarchy::ChildSpawnerCommands;
use bevy::prelude::*;

use crate::capabilities::particles::DemoParticleEmitter2d;
use crate::primitives::frame_animation::FrameAnimationManifest2d;
use crate::products::overlays::DemoHealthBar2d;

use super::player_sprite::DemoPlayerSprite2d;

pub struct DemoPlayerVisual2d {
    frame_manifest: Handle<FrameAnimationManifest2d>,
}

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
struct DemoPlayerVisual2dMarker;

#[derive(Bundle)]
struct DemoPlayerVisual2dRootBundle {
    marker: DemoPlayerVisual2dMarker,
    transform: Transform,
    visibility: Visibility,
}

impl Default for DemoPlayerVisual2dRootBundle {
    fn default() -> Self {
        Self {
            marker: DemoPlayerVisual2dMarker,
            transform: Transform::default(),
            visibility: Visibility::default(),
        }
    }
}

impl DemoPlayerVisual2d {
    pub fn new(frame_manifest: Handle<FrameAnimationManifest2d>) -> Self {
        Self { frame_manifest }
    }

    pub fn spawn(self, parent: &mut ChildSpawnerCommands) {
        parent
            .spawn(DemoPlayerVisual2dRootBundle::default())
            .with_children(|visual| {
                visual.spawn(DemoPlayerSprite2d::new(self.frame_manifest));
                visual.spawn(DemoParticleEmitter2d::default());
                DemoHealthBar2d::spawn(visual);
            });
    }
}
