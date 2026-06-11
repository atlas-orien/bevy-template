use std::collections::HashMap;

use gameplay::api::{
    RuntimeEntityRegistrationMessage, RuntimeObjectId, RuntimeSpawnContext, RuntimeUserId,
};
use prefab::identity::GameplayEntityId;

#[derive(Debug, Default)]
pub struct ManagerState {
    entities: HashMap<GameplayEntityId, RuntimeEntityRegistrationMessage>,
    next_gameplay_entity_id: u64,
}

impl ManagerState {
    pub fn allocate_registration(
        &mut self,
        context: RuntimeSpawnContext,
    ) -> RuntimeEntityRegistrationMessage {
        let id = self.allocate_gameplay_entity_id();
        RuntimeEntityRegistrationMessage::new(id, context)
    }

    pub fn register_entity(&mut self, registration: RuntimeEntityRegistrationMessage) {
        self.next_gameplay_entity_id = self
            .next_gameplay_entity_id
            .max(registration.gameplay_entity_id.0.saturating_add(1));
        self.entities
            .insert(registration.gameplay_entity_id, registration);
    }

    pub fn unregister_entity(&mut self, id: GameplayEntityId) {
        self.entities.remove(&id);
    }

    pub fn entities(&self) -> Vec<RuntimeEntityRegistrationMessage> {
        self.entities.values().copied().collect()
    }

    pub fn entities_for_user(
        &self,
        owner_user_id: RuntimeUserId,
    ) -> Vec<RuntimeEntityRegistrationMessage> {
        self.entities
            .values()
            .filter(|registration| registration.owner_user_id == Some(owner_user_id))
            .copied()
            .collect()
    }

    pub fn entities_for_object(
        &self,
        external_object_id: RuntimeObjectId,
    ) -> Vec<RuntimeEntityRegistrationMessage> {
        self.entities
            .values()
            .filter(|registration| registration.external_object_id == Some(external_object_id))
            .copied()
            .collect()
    }

    pub fn has_user_entity(&self, owner_user_id: RuntimeUserId) -> bool {
        self.entities
            .values()
            .any(|registration| registration.owner_user_id == Some(owner_user_id))
    }

    pub fn has_object_entity(&self, external_object_id: RuntimeObjectId) -> bool {
        self.entities
            .values()
            .any(|registration| registration.external_object_id == Some(external_object_id))
    }

    pub fn clear(&mut self) {
        self.entities.clear();
    }

    fn allocate_gameplay_entity_id(&mut self) -> GameplayEntityId {
        if self.next_gameplay_entity_id == 0 {
            self.next_gameplay_entity_id = 1;
        }

        while self
            .entities
            .contains_key(&GameplayEntityId(self.next_gameplay_entity_id))
        {
            self.next_gameplay_entity_id = self.next_gameplay_entity_id.saturating_add(1);
        }

        let id = GameplayEntityId(self.next_gameplay_entity_id);
        self.next_gameplay_entity_id = self.next_gameplay_entity_id.saturating_add(1);
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn registration(
        id: u64,
        user: Option<u64>,
        object: Option<u64>,
    ) -> RuntimeEntityRegistrationMessage {
        RuntimeEntityRegistrationMessage {
            gameplay_entity_id: GameplayEntityId(id),
            owner_user_id: user.map(RuntimeUserId),
            external_object_id: object.map(RuntimeObjectId),
        }
    }

    #[test]
    fn stores_full_registration_payload() {
        let mut state = ManagerState::default();
        let record = registration(7, Some(11), Some(13));

        state.register_entity(record);

        assert_eq!(state.entities(), vec![record]);
        assert_eq!(state.entities_for_user(RuntimeUserId(11)), vec![record]);
        assert_eq!(state.entities_for_object(RuntimeObjectId(13)), vec![record]);
    }

    #[test]
    fn unregister_removes_all_query_views() {
        let mut state = ManagerState::default();
        state.register_entity(registration(7, Some(11), Some(13)));

        state.unregister_entity(GameplayEntityId(7));

        assert!(state.entities().is_empty());
        assert!(!state.has_user_entity(RuntimeUserId(11)));
        assert!(!state.has_object_entity(RuntimeObjectId(13)));
    }

    #[test]
    fn allocates_internal_gameplay_ids_for_runtime_context() {
        let mut state = ManagerState::default();

        let registration = state.allocate_registration(RuntimeSpawnContext::for_user_object(
            RuntimeUserId(11),
            RuntimeObjectId(13),
        ));

        assert_eq!(registration.gameplay_entity_id, GameplayEntityId(1));
        assert_eq!(registration.owner_user_id, Some(RuntimeUserId(11)));
        assert_eq!(registration.external_object_id, Some(RuntimeObjectId(13)));
    }
}
