use bevy::prelude::*;
use prefab::Prefab;
use prefab::ui::{DemoMenuPrefab, UiCameraPrefab};

use super::plan::GameplaySpawnPlan;

pub fn default_gameplay_spawn_plan() -> GameplaySpawnPlan {
    GameplaySpawnPlan::new()
}

pub fn spawn_initial_gameplay_plan_system(mut commands: Commands) {
    for prefab in default_gameplay_spawn_plan().into_prefabs() {
        prefab.spawn_boxed(&mut commands);
    }

    let ui_camera = UiCameraPrefab.spawn(&mut commands);
    DemoMenuPrefab { ui_camera }.spawn(&mut commands);

    info!("Initial gameplay spawn plan completed.");
}
