use bevy::prelude::*;

use super::player::demo_player_animation_state_system;

pub(in crate::animation::frame) struct FrameAnimationDemoPlugin;

impl Plugin for FrameAnimationDemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, demo_player_animation_state_system);
    }
}
