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

    pub fn drain_updates(&self) -> Vec<RuntimeUpdateMessage> {
        self.updates.drain()
    }

    pub fn apply_update(&self, update: RuntimeUpdateMessage) {
        match update {
            RuntimeUpdateMessage::EntityRegistered(registration) => {
                if let Ok(mut state) = self.state.lock() {
                    state.register_entity(registration);
                }
            }
            RuntimeUpdateMessage::EntityUnregistered { gameplay_entity_id } => {
                if let Ok(mut state) = self.state.lock() {
                    state.unregister_entity(gameplay_entity_id);
                }
            }
            RuntimeUpdateMessage::DemoNetworkLoginRequested => {}
        }
    }
}
