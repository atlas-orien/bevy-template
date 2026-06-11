use std::sync::{Arc, Mutex};

use gameplay::api::{
    RuntimeRequestMessage, RuntimeRequestSender, RuntimeUpdateInbox, RuntimeUpdateMessage,
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

    pub fn send_request(&self, request: RuntimeRequestMessage) -> bool {
        self.requests.send(request)
    }

    pub fn receive_updates(&self) {
        for update in self.updates.drain() {
            let Ok(mut state) = self.state.lock() else {
                return;
            };

            match update {
                RuntimeUpdateMessage::EntityRegistered(registration) => {
                    state.register_entity(registration);
                }
                RuntimeUpdateMessage::EntityUnregistered { gameplay_entity_id } => {
                    state.unregister_entity(gameplay_entity_id);
                }
            }
        }
    }
}
