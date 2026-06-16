use bevy::prelude::*;

use super::FrameAnimationBasePlugin;
use super::demo_player::demo_player_animation_state_system;

pub struct FrameAnimation2dPlugin;

impl Plugin for FrameAnimation2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameAnimationBasePlugin)
            .add_systems(Update, demo_player_animation_state_system);
    }
}
