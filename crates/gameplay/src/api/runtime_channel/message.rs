use bevy::prelude::*;
use prefab::Prefab;
use prefab::identity::GameplayEntityId;

use super::spawn::SpawnItem;
use crate::state::AppState;

#[derive(Message)]
pub enum RuntimeRequestMessage {
    SpawnPrefab(Option<Box<dyn SpawnItem>>),
    DespawnEntity(GameplayEntityId),
    ClearSession,
    ChangeState(AppState),
    SetMovementIntent {
        id: GameplayEntityId,
        target: intent::movement::MovementTarget,
    },
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct RuntimeUserId(pub u64);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct RuntimeObjectId(pub u64);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct RuntimeEntityRegistrationMessage {
    pub gameplay_entity_id: GameplayEntityId,
    pub owner_user_id: Option<RuntimeUserId>,
    pub external_object_id: Option<RuntimeObjectId>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RuntimeUpdateMessage {
    EntityRegistered(RuntimeEntityRegistrationMessage),
    EntityUnregistered {
        gameplay_entity_id: GameplayEntityId,
    },
}

impl RuntimeRequestMessage {
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

impl RuntimeUpdateMessage {
    pub fn entity_registered(gameplay_entity_id: GameplayEntityId) -> Self {
        Self::EntityRegistered(RuntimeEntityRegistrationMessage {
            gameplay_entity_id,
            owner_user_id: None,
            external_object_id: None,
        })
    }

    pub fn entity_unregistered(gameplay_entity_id: GameplayEntityId) -> Self {
        Self::EntityUnregistered { gameplay_entity_id }
    }
}
