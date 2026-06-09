use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::{self, Receiver, Sender};

use bevy::prelude::*;

use super::request::GameplayRequest;

#[derive(Clone)]
pub struct GameplayManager {
    requests: Sender<GameplayRequest>,
}

#[derive(Clone)]
pub struct GameplayRequestInbox {
    requests: Arc<Mutex<Receiver<GameplayRequest>>>,
}

impl Resource for GameplayRequestInbox {}

impl GameplayManager {
    pub fn new() -> (Self, GameplayRequestInbox) {
        let (requests, inbox) = mpsc::channel();

        (
            Self { requests },
            GameplayRequestInbox {
                requests: Arc::new(Mutex::new(inbox)),
            },
        )
    }

    pub fn submit(&self, request: GameplayRequest) -> bool {
        self.requests.send(request).is_ok()
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
