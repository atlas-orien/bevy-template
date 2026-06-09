use bevy::prelude::*;
use prefab::Prefab;
use prefab::world_2d::characters::player::Player2dPrefab;

use crate::state::AppState;

pub struct SpawningPlugin;

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), enter_main_menu)
            .add_systems(OnEnter(AppState::Playing), enter_playing);
    }
}

fn enter_main_menu() {
    info!("Main menu runtime ready.");
}

fn enter_playing(mut commands: Commands) {
    Player2dPrefab::default().spawn(&mut commands);
    info!("Playing runtime ready.");
}
