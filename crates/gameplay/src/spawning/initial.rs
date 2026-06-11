use bevy::prelude::*;
use prefab::Prefab;
use prefab::ui::DemoMenuPrefab;
use render_2d::camera::UiCamera;

use super::plan::GameplaySpawnPlan;

pub fn default_gameplay_spawn_plan() -> GameplaySpawnPlan {
    GameplaySpawnPlan::new()
}

pub fn spawn_initial_gameplay_plan_system(mut commands: Commands) {
    for prefab in default_gameplay_spawn_plan().into_prefabs() {
        prefab.spawn_boxed(&mut commands);
    }

    let ui_camera = commands.spawn(UiCamera::default()).id();
    let menu = DemoMenuPrefab.spawn(&mut commands);
    commands.entity(menu).insert(UiTargetCamera(ui_camera));

    info!("Initial gameplay spawn plan completed.");
}
