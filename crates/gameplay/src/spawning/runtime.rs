use bevy::prelude::*;

use crate::api::SpawnItem;

pub fn spawn_runtime_prefab(commands: &mut Commands, prefab: Box<dyn SpawnItem>) -> Entity {
    prefab.spawn_boxed(commands)
}
