use bevy::prelude::*;
use prefab::Prefab;

use crate::spawning::prefab::SpawnItem;

#[derive(Message)]
pub enum GameplayRequest {
    SpawnPrefab(Option<Box<dyn SpawnItem>>),
}

impl GameplayRequest {
    pub fn spawn_prefab<P>(prefab: P) -> Self
    where
        P: Prefab + Send + Sync + 'static,
    {
        Self::SpawnPrefab(Some(Box::new(prefab)))
    }
}
