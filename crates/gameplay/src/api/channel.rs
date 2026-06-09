use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};

use bevy::prelude::*;

use super::request::GameplayRequest;

#[derive(Clone)]
pub struct GameplayRequestSender {
    requests: Sender<GameplayRequest>,
}

#[derive(Clone)]
pub struct GameplayUpdateSender {
    updates: Sender<GameplayUpdate>,
}

#[derive(Clone)]
pub struct GameplayRequestInbox {
    requests: Arc<Mutex<Receiver<GameplayRequest>>>,
}

#[derive(Clone)]
pub struct GameplayUpdateInbox {
    updates: Arc<Mutex<Receiver<GameplayUpdate>>>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GameplayUpdate {
    EntityRegistered {
        id: prefab::identity::GameplayEntityId,
    },
    EntityUnregistered {
        id: prefab::identity::GameplayEntityId,
    },
}

impl Resource for GameplayRequestInbox {}
impl Resource for GameplayUpdateSender {}

pub fn gameplay_channels() -> (
    GameplayRequestSender,
    GameplayRequestInbox,
    GameplayUpdateSender,
    GameplayUpdateInbox,
) {
    let (requests, inbox) = mpsc::channel();
    let (updates, update_inbox) = mpsc::channel();

    (
        GameplayRequestSender { requests },
        GameplayRequestInbox {
            requests: Arc::new(Mutex::new(inbox)),
        },
        GameplayUpdateSender { updates },
        GameplayUpdateInbox {
            updates: Arc::new(Mutex::new(update_inbox)),
        },
    )
}

impl GameplayRequestSender {
    pub fn submit(&self, request: GameplayRequest) -> bool {
        self.requests.send(request).is_ok()
    }
}

impl GameplayUpdateSender {
    pub fn submit(&self, update: GameplayUpdate) -> bool {
        self.updates.send(update).is_ok()
    }
}

impl GameplayRequestInbox {
    pub fn drain_into(&self, requests: &mut MessageWriter<GameplayRequest>) {
        let Ok(inbox) = self.requests.lock() else {
            return;
        };

        for request in inbox.try_iter() {
            requests.write(request);
        }
    }
}

impl GameplayUpdateInbox {
    pub fn drain(&self) -> Vec<GameplayUpdate> {
        let Ok(inbox) = self.updates.lock() else {
            return Vec::new();
        };

        inbox.try_iter().collect()
    }
}
