use bevy::prelude::*;

use super::systems::apply_pose_to_bones_system;

pub struct Skeleton3dPlugin;

impl Plugin for Skeleton3dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_pose_to_bones_system);
    }
}
