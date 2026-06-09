use bevy::prelude::*;

use crate::api::request::GameplayRequest;

use super::defaults::default_gameplay_spawn_plan;

pub fn spawn_gameplay_plan_system(mut commands: Commands) {
    for prefab in default_gameplay_spawn_plan().into_prefabs() {
        prefab.spawn_boxed(&mut commands);
    }

    info!("Playing gameplay spawned.");
}

pub fn spawn_requested_prefabs_system(
    mut commands: Commands,
    mut requests: MessageMutator<GameplayRequest>,
) {
    for request in requests.read() {
        match request {
            GameplayRequest::SpawnPrefab(prefab) => {
                if let Some(prefab) = prefab.take() {
                    prefab.spawn_boxed(&mut commands);
                }
            }
        };
    }
}
