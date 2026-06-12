pub mod demo_frame_animation;
pub mod example;

use bevy::prelude::*;

pub use demo_frame_animation::{
    DemoFrameAnimation2d, DemoPlayerAnimation2d, demo_frame_animation_system,
    demo_player_animation_state_system,
};

pub struct FrameAnimation2dPlugin;

impl Plugin for FrameAnimation2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                demo_player_animation_state_system,
                demo_frame_animation_system,
            )
                .chain(),
        );
    }
}
