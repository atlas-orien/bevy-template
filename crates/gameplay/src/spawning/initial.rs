use bevy::prelude::*;
use prefab::world_2d::characters::player::Player2dPrefab;

use super::plan::GameplaySpawnPlan;

pub fn default_gameplay_spawn_plan() -> GameplaySpawnPlan {
    // Example spawn plan with a single player entity. In a real game, this would likely be loaded from a file or generated procedurally.
    GameplaySpawnPlan::new().with(Player2dPrefab::default())
}

pub fn spawn_initial_gameplay_plan_system(mut commands: Commands) {
    for prefab in default_gameplay_spawn_plan().into_prefabs() {
        prefab.spawn_boxed(&mut commands);
    }

    info!("Initial gameplay spawn plan completed.");
}
