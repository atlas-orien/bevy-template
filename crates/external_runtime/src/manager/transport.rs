use std::sync::{Arc, Mutex};

use gameplay::api::{
    RuntimeRequest, RuntimeRequestSender, RuntimeUpdate, RuntimeUpdateInbox,
};

use super::state::ManagerState;

#[derive(Clone)]
pub struct RuntimeTransport {
    requests: RuntimeRequestSender,
    updates: RuntimeUpdateInbox,
    state: Arc<Mutex<ManagerState>>,
}

impl RuntimeTransport {
    pub fn new(
        requests: RuntimeRequestSender,
        updates: RuntimeUpdateInbox,
        state: Arc<Mutex<ManagerState>>,
    ) -> Self {
        Self {
            requests,
            updates,
            state,
        }
    }

    pub fn send_request(&self, request: RuntimeRequest) -> bool {
        self.requests.send(request)
    }

    pub fn receive_updates(&self) {
        for update in self.updates.drain() {
            let Ok(mut state) = self.state.lock() else {
                return;
            };

            match update {
                RuntimeUpdate::EntityRegistered { id } => {
                    state.register_entity(id);
                }
                RuntimeUpdate::EntityUnregistered { id } => {
                    state.unregister_entity(id);
                }
            }
        }
    }
}
