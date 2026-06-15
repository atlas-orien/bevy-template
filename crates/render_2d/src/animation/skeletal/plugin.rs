use bevy::prelude::*;

use super::demo_skeletal_animation::demo_skeletal_animation_system;

pub struct SkeletalAnimation2dPlugin;

impl Plugin for SkeletalAnimation2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, demo_skeletal_animation_system);
    }
}
