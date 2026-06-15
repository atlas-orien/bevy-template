use std::sync::{Arc, Mutex};

use gameplay::api::{
    RuntimeEntityRegistrationMessage, RuntimeObjectId, RuntimeRequestMessage, RuntimeRequestSender,
    RuntimeSpawnContext, RuntimeUpdateInbox, RuntimeUpdateMessage, RuntimeUserId,
};
use gameplay::state::AppState;
use intent::movement::MovementTarget;
use prefab::Prefab;

use super::state::ManagerState;
use super::transport::RuntimeTransport;

#[derive(Clone)]
pub struct ExternalRuntimeManager {
    runtime: RuntimeTransport,
    state: Arc<Mutex<ManagerState>>,
}

impl ExternalRuntimeManager {
    pub fn new(requests: RuntimeRequestSender, updates: RuntimeUpdateInbox) -> Self {
        let state = Arc::new(Mutex::new(ManagerState::default()));
        let runtime = RuntimeTransport::new(requests, updates, state.clone());

        Self { runtime, state }
    }

    pub(crate) fn drain_gameplay_updates(&self) -> Vec<RuntimeUpdateMessage> {
        self.runtime.drain_updates()
    }

    pub(crate) fn apply_gameplay_update(&self, update: RuntimeUpdateMessage) {
        self.runtime.apply_update(update);
    }

    pub fn entities(&self) -> Vec<RuntimeEntityRegistrationMessage> {
        self.state
            .lock()
            .map(|state| state.entities())
            .unwrap_or_default()
    }

    pub fn entities_for_user(
        &self,
        owner_user_id: RuntimeUserId,
    ) -> Vec<RuntimeEntityRegistrationMessage> {
        self.state
            .lock()
            .map(|state| state.entities_for_user(owner_user_id))
            .unwrap_or_default()
    }

    pub fn entities_for_object(
        &self,
        external_object_id: RuntimeObjectId,
    ) -> Vec<RuntimeEntityRegistrationMessage> {
        self.state
            .lock()
            .map(|state| state.entities_for_object(external_object_id))
            .unwrap_or_default()
    }

    pub fn has_user_entity(&self, owner_user_id: RuntimeUserId) -> bool {
        self.state
            .lock()
            .is_ok_and(|state| state.has_user_entity(owner_user_id))
    }

    pub fn has_object_entity(&self, external_object_id: RuntimeObjectId) -> bool {
        self.state
            .lock()
            .is_ok_and(|state| state.has_object_entity(external_object_id))
    }
}

pub fn spawn_prefab<P>(manager: &ExternalRuntimeManager, prefab: P) -> bool
where
    P: Prefab + Send + Sync + 'static,
{
    spawn_prefab_with_context(manager, prefab, RuntimeSpawnContext::new(None, None))
}

pub fn spawn_prefab_for_user<P>(
    manager: &ExternalRuntimeManager,
    owner_user_id: RuntimeUserId,
    prefab: P,
) -> bool
where
    P: Prefab + Send + Sync + 'static,
{
    spawn_prefab_with_context(
        manager,
        prefab,
        RuntimeSpawnContext::for_user(owner_user_id),
    )
}

pub fn spawn_prefab_for_object<P>(
    manager: &ExternalRuntimeManager,
    external_object_id: RuntimeObjectId,
    prefab: P,
) -> bool
where
    P: Prefab + Send + Sync + 'static,
{
    spawn_prefab_with_context(
        manager,
        prefab,
        RuntimeSpawnContext::for_object(external_object_id),
    )
}

pub fn spawn_prefab_for_user_object<P>(
    manager: &ExternalRuntimeManager,
    owner_user_id: RuntimeUserId,
    external_object_id: RuntimeObjectId,
    prefab: P,
) -> bool
where
    P: Prefab + Send + Sync + 'static,
{
    spawn_prefab_with_context(
        manager,
        prefab,
        RuntimeSpawnContext::for_user_object(owner_user_id, external_object_id),
    )
}

pub fn spawn_prefab_with_context<P>(
    manager: &ExternalRuntimeManager,
    prefab: P,
    context: RuntimeSpawnContext,
) -> bool
where
    P: Prefab + Send + Sync + 'static,
{
    let Some(registration) = manager
        .state
        .lock()
        .ok()
        .map(|mut state| state.allocate_registration(context))
    else {
        return false;
    };

    manager
        .runtime
        .send_request(RuntimeRequestMessage::spawn_prefab(prefab, registration))
}

pub fn despawn_user_entities(
    manager: &ExternalRuntimeManager,
    owner_user_id: RuntimeUserId,
) -> bool {
    send_for_registrations(
        manager,
        manager.entities_for_user(owner_user_id),
        |registration| RuntimeRequestMessage::despawn_entity(registration.gameplay_entity_id),
    )
}

pub fn despawn_object_entities(
    manager: &ExternalRuntimeManager,
    external_object_id: RuntimeObjectId,
) -> bool {
    send_for_registrations(
        manager,
        manager.entities_for_object(external_object_id),
        |registration| RuntimeRequestMessage::despawn_entity(registration.gameplay_entity_id),
    )
}

pub fn clear_session(manager: &ExternalRuntimeManager) -> bool {
    if let Ok(mut state) = manager.state.lock() {
        state.clear();
    }

    manager
        .runtime
        .send_request(RuntimeRequestMessage::clear_session())
}

pub fn change_state(manager: &ExternalRuntimeManager, state: AppState) -> bool {
    manager
        .runtime
        .send_request(RuntimeRequestMessage::change_state(state))
}

pub fn set_user_movement_intent(
    manager: &ExternalRuntimeManager,
    owner_user_id: RuntimeUserId,
    target: MovementTarget,
) -> bool {
    send_for_registrations(
        manager,
        manager.entities_for_user(owner_user_id),
        |registration| {
            RuntimeRequestMessage::set_movement_intent(registration.gameplay_entity_id, target)
        },
    )
}

pub fn set_object_movement_intent(
    manager: &ExternalRuntimeManager,
    external_object_id: RuntimeObjectId,
    target: MovementTarget,
) -> bool {
    send_for_registrations(
        manager,
        manager.entities_for_object(external_object_id),
        |registration| {
            RuntimeRequestMessage::set_movement_intent(registration.gameplay_entity_id, target)
        },
    )
}

pub fn entities(manager: &ExternalRuntimeManager) -> Vec<RuntimeEntityRegistrationMessage> {
    manager.entities()
}

pub fn entities_for_user(
    manager: &ExternalRuntimeManager,
    owner_user_id: RuntimeUserId,
) -> Vec<RuntimeEntityRegistrationMessage> {
    manager.entities_for_user(owner_user_id)
}

pub fn entities_for_object(
    manager: &ExternalRuntimeManager,
    external_object_id: RuntimeObjectId,
) -> Vec<RuntimeEntityRegistrationMessage> {
    manager.entities_for_object(external_object_id)
}

pub fn has_user_entity(manager: &ExternalRuntimeManager, owner_user_id: RuntimeUserId) -> bool {
    manager.has_user_entity(owner_user_id)
}

pub fn has_object_entity(
    manager: &ExternalRuntimeManager,
    external_object_id: RuntimeObjectId,
) -> bool {
    manager.has_object_entity(external_object_id)
}

fn send_for_registrations(
    manager: &ExternalRuntimeManager,
    registrations: Vec<RuntimeEntityRegistrationMessage>,
    to_request: impl Fn(RuntimeEntityRegistrationMessage) -> RuntimeRequestMessage,
) -> bool {
    if registrations.is_empty() {
        return false;
    }

    registrations
        .into_iter()
        .all(|registration| manager.runtime.send_request(to_request(registration)))
}
