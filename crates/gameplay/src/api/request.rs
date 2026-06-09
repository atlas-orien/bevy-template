use bevy::prelude::*;
use prefab::Prefab;
use prefab::identity::GameplayEntityId;

use crate::spawning::prefab::SpawnItem;
use crate::state::AppState;

#[derive(Message)]
pub enum GameplayRequest {
    SpawnPrefab(Option<Box<dyn SpawnItem>>),
    DespawnEntity(GameplayEntityId),
    ClearSession,
    ChangeState(AppState),
    SetMovementIntent {
        id: GameplayEntityId,
        target: intent::movement::MovementTarget,
    },
}

impl GameplayRequest {
    pub fn spawn_prefab<P>(prefab: P) -> Self
    where
        P: Prefab + Send + Sync + 'static,
    {
        Self::SpawnPrefab(Some(Box::new(prefab)))
    }

    pub fn despawn_entity(id: GameplayEntityId) -> Self {
        Self::DespawnEntity(id)
    }

    pub fn clear_session() -> Self {
        Self::ClearSession
    }

    pub fn change_state(state: AppState) -> Self {
        Self::ChangeState(state)
    }

    pub fn set_movement_intent(
        id: GameplayEntityId,
        target: intent::movement::MovementTarget,
    ) -> Self {
        Self::SetMovementIntent { id, target }
    }
}
