use std::sync::{Arc, Mutex};

use gameplay::api::{RuntimeRequest, RuntimeRequestSender, RuntimeUpdateInbox};
use gameplay::state::AppState;
use intent::movement::MovementTarget;
use prefab::Prefab;
use prefab::identity::GameplayEntityId;

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

    pub(crate) fn sync_gameplay_updates(&self) {
        self.runtime.receive_updates();
    }

    pub fn has_entity(&self, id: GameplayEntityId) -> bool {
        self.state.lock().is_ok_and(|state| state.has_entity(id))
    }

    pub fn entity_ids(&self) -> Vec<GameplayEntityId> {
        self.state
            .lock()
            .map(|state| state.entity_ids())
            .unwrap_or_default()
    }
}

pub fn spawn_prefab<P>(manager: &ExternalRuntimeManager, prefab: P) -> bool
where
    P: Prefab + Send + Sync + 'static,
{
    manager
        .runtime
        .send_request(RuntimeRequest::spawn_prefab(prefab))
}

pub fn despawn_entity(manager: &ExternalRuntimeManager, id: GameplayEntityId) -> bool {
    manager
        .runtime
        .send_request(RuntimeRequest::despawn_entity(id))
}

pub fn clear_session(manager: &ExternalRuntimeManager) -> bool {
    if let Ok(mut state) = manager.state.lock() {
        state.clear();
    }

    manager
        .runtime
        .send_request(RuntimeRequest::clear_session())
}

pub fn change_state(manager: &ExternalRuntimeManager, state: AppState) -> bool {
    manager
        .runtime
        .send_request(RuntimeRequest::change_state(state))
}

pub fn set_movement_intent(
    manager: &ExternalRuntimeManager,
    id: GameplayEntityId,
    target: MovementTarget,
) -> bool {
    manager
        .runtime
        .send_request(RuntimeRequest::set_movement_intent(id, target))
}

pub fn has_entity(manager: &ExternalRuntimeManager, id: GameplayEntityId) -> bool {
    manager.has_entity(id)
}

pub fn entity_ids(manager: &ExternalRuntimeManager) -> Vec<GameplayEntityId> {
    manager.entity_ids()
}
