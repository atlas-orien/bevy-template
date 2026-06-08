use bevy::prelude::*;
use scenes::shared::despawn_scene_entities;

use crate::state::AppState;

pub struct CleanupPlugin;

impl Plugin for CleanupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Playing), despawn_scene_entities);
    }
}
