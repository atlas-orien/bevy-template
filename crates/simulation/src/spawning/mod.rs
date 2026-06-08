use bevy::prelude::*;
use scenes::level_01::spawn_level_01_scene;
use scenes::main_menu::spawn_main_menu_scene;

use crate::state::AppState;

pub struct SpawningPlugin;

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu_scene)
            .add_systems(OnEnter(AppState::Playing), spawn_level_01_scene);
    }
}
