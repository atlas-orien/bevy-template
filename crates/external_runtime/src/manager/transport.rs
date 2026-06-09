use std::sync::{Arc, Mutex};

use gameplay::api::{GameplayRequest, GameplayRequestSender, GameplayUpdate, GameplayUpdateInbox};

use super::state::ManagerState;

#[derive(Clone)]
pub struct GameplayTransport {
    requests: GameplayRequestSender,
    updates: GameplayUpdateInbox,
    state: Arc<Mutex<ManagerState>>,
}

impl GameplayTransport {
    pub fn new(
        requests: GameplayRequestSender,
        updates: GameplayUpdateInbox,
        state: Arc<Mutex<ManagerState>>,
    ) -> Self {
        Self {
            requests,
            updates,
            state,
        }
    }

    pub fn send_request(&self, request: GameplayRequest) -> bool {
        self.requests.submit(request)
    }

    pub fn receive_updates(&self) {
        for update in self.updates.drain() {
            let Ok(mut state) = self.state.lock() else {
                return;
            };

            match update {
                GameplayUpdate::EntityRegistered { id } => {
                    state.register_entity(id);
                }
                GameplayUpdate::EntityUnregistered { id } => {
                    state.unregister_entity(id);
                }
            }
        }
    }
}
