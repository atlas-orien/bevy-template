use bevy::prelude::*;
use prefab::Prefab;

use crate::spawning::prefab::SpawnItem;
use crate::state::AppState;

#[derive(Message)]
pub enum GameplayRequest {
    SpawnPrefab(Option<Box<dyn SpawnItem>>),
    DespawnEntity(Entity),
    ClearSession,
    ChangeState(AppState),
}

impl GameplayRequest {
    pub fn spawn_prefab<P>(prefab: P) -> Self
    where
        P: Prefab + Send + Sync + 'static,
    {
        Self::SpawnPrefab(Some(Box::new(prefab)))
    }

    pub fn despawn_entity(entity: Entity) -> Self {
        Self::DespawnEntity(entity)
    }

    pub fn clear_session() -> Self {
        Self::ClearSession
    }

    pub fn change_state(state: AppState) -> Self {
        Self::ChangeState(state)
    }
}
