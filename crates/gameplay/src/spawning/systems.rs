use bevy::prelude::*;

use super::defaults::default_gameplay_spawn_plan;

pub fn spawn_gameplay_plan_system(mut commands: Commands) {
    for prefab in default_gameplay_spawn_plan().into_prefabs() {
        prefab.spawn_boxed(&mut commands);
    }

    info!("Playing gameplay spawned.");
}
