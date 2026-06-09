use std::sync::{Arc, Mutex};

use gameplay::api::{
    GameplayRequest, GameplayRequestInbox, GameplayUpdateSender, gameplay_channels,
};
use gameplay::state::AppState;
use intent::movement::MovementTarget;
use prefab::Prefab;
use prefab::identity::GameplayEntityId;

use super::state::ManagerState;
use super::transport::GameplayTransport;

#[derive(Clone)]
pub struct ExternalRuntimeManager {
    gameplay: GameplayTransport,
    state: Arc<Mutex<ManagerState>>,
    request_inbox: GameplayRequestInbox,
    update_sender: GameplayUpdateSender,
}

impl ExternalRuntimeManager {
    pub fn new() -> Self {
        let (requests, request_inbox, update_sender, updates) = gameplay_channels();
        let state = Arc::new(Mutex::new(ManagerState::default()));
        let gameplay = GameplayTransport::new(requests, updates, state.clone());

        Self {
            gameplay,
            state,
            request_inbox,
            update_sender,
        }
    }

    pub(crate) fn sync_gameplay_updates(&self) {
        self.gameplay.receive_updates();
    }

    pub fn gameplay_request_inbox(&self) -> GameplayRequestInbox {
        self.request_inbox.clone()
    }

    pub fn gameplay_update_sender(&self) -> GameplayUpdateSender {
        self.update_sender.clone()
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

impl Default for ExternalRuntimeManager {
    fn default() -> Self {
        Self::new()
    }
}

pub fn spawn_prefab<P>(manager: &ExternalRuntimeManager, prefab: P) -> bool
where
    P: Prefab + Send + Sync + 'static,
{
    manager
        .gameplay
        .send_request(GameplayRequest::spawn_prefab(prefab))
}

pub fn despawn_entity(manager: &ExternalRuntimeManager, id: GameplayEntityId) -> bool {
    manager
        .gameplay
        .send_request(GameplayRequest::despawn_entity(id))
}

pub fn clear_session(manager: &ExternalRuntimeManager) -> bool {
    if let Ok(mut state) = manager.state.lock() {
        state.clear();
    }

    manager
        .gameplay
        .send_request(GameplayRequest::clear_session())
}

pub fn change_state(manager: &ExternalRuntimeManager, state: AppState) -> bool {
    manager
        .gameplay
        .send_request(GameplayRequest::change_state(state))
}

pub fn set_movement_intent(
    manager: &ExternalRuntimeManager,
    id: GameplayEntityId,
    target: MovementTarget,
) -> bool {
    manager
        .gameplay
        .send_request(GameplayRequest::set_movement_intent(id, target))
}

pub fn has_entity(manager: &ExternalRuntimeManager, id: GameplayEntityId) -> bool {
    manager.has_entity(id)
}

pub fn entity_ids(manager: &ExternalRuntimeManager) -> Vec<GameplayEntityId> {
    manager.entity_ids()
}
