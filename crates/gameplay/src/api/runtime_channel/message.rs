use bevy::prelude::*;
use prefab::Prefab;
use prefab::identity::GameplayEntityId;

use super::spawn::SpawnItem;
use crate::state::AppState;

#[derive(Message)]
pub enum RuntimeRequestMessage {
    SpawnPrefab(RuntimeSpawnRequestMessage),
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
pub struct RuntimeSpawnContext {
    pub owner_user_id: Option<RuntimeUserId>,
    pub external_object_id: Option<RuntimeObjectId>,
}

impl RuntimeSpawnContext {
    pub const fn new(
        owner_user_id: Option<RuntimeUserId>,
        external_object_id: Option<RuntimeObjectId>,
    ) -> Self {
        Self {
            owner_user_id,
            external_object_id,
        }
    }

    pub const fn for_user(owner_user_id: RuntimeUserId) -> Self {
        Self::new(Some(owner_user_id), None)
    }

    pub const fn for_object(external_object_id: RuntimeObjectId) -> Self {
        Self::new(None, Some(external_object_id))
    }

    pub const fn for_user_object(
        owner_user_id: RuntimeUserId,
        external_object_id: RuntimeObjectId,
    ) -> Self {
        Self::new(Some(owner_user_id), Some(external_object_id))
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct RuntimeEntityRegistrationMessage {
    pub gameplay_entity_id: GameplayEntityId,
    pub owner_user_id: Option<RuntimeUserId>,
    pub external_object_id: Option<RuntimeObjectId>,
}

impl RuntimeEntityRegistrationMessage {
    pub const fn new(gameplay_entity_id: GameplayEntityId, context: RuntimeSpawnContext) -> Self {
        Self {
            gameplay_entity_id,
            owner_user_id: context.owner_user_id,
            external_object_id: context.external_object_id,
        }
    }
}

pub struct RuntimeSpawnRequestMessage {
    pub prefab: Option<Box<dyn SpawnItem>>,
    pub registration: RuntimeEntityRegistrationMessage,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RuntimeUpdateMessage {
    EntityRegistered(RuntimeEntityRegistrationMessage),
    EntityUnregistered {
        gameplay_entity_id: GameplayEntityId,
    },
    DemoNetworkLoginRequested,
}

impl RuntimeRequestMessage {
    pub fn spawn_prefab<P>(prefab: P, registration: RuntimeEntityRegistrationMessage) -> Self
    where
        P: Prefab + Send + Sync + 'static,
    {
        Self::SpawnPrefab(RuntimeSpawnRequestMessage {
            prefab: Some(Box::new(prefab)),
            registration,
        })
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

    pub fn demo_network_login_requested() -> Self {
        Self::DemoNetworkLoginRequested
    }
}
