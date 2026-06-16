use bevy::prelude::*;

use super::demo_frame_animation::{
    demo_frame_animation_system, demo_player_animation_state_system,
};
use super::demo_frame_manifest::{DemoFrameManifest2d, DemoFrameManifestLoader2d};

pub struct FrameAnimation2dPlugin;

impl Plugin for FrameAnimation2dPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<DemoFrameManifest2d>()
            .init_asset_loader::<DemoFrameManifestLoader2d>()
            .add_systems(
                Update,
                (
                    demo_player_animation_state_system,
                    demo_frame_animation_system,
                )
                    .chain(),
            );
    }
}
