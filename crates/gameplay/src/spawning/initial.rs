use bevy::prelude::*;

use super::plan::GameplaySpawnPlan;

pub fn default_gameplay_spawn_plan() -> GameplaySpawnPlan {
    GameplaySpawnPlan::new()
}

pub fn spawn_initial_gameplay_plan_system(mut commands: Commands) {
    for prefab in default_gameplay_spawn_plan().into_prefabs() {
        prefab.spawn_boxed(&mut commands);
    }

    info!("Initial gameplay spawn plan completed.");
}
