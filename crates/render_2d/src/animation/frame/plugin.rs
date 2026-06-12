use bevy::prelude::*;

use super::demo_frame_animation::{
    demo_frame_animation_system, demo_player_animation_state_system,
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
