use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};

use bevy::prelude::*;

use super::request::GameplayRequest;

#[derive(Clone)]
pub struct GameplayRequestSender {
    requests: Sender<GameplayRequest>,
}

#[derive(Clone)]
pub struct GameplayRequestInbox {
    requests: Arc<Mutex<Receiver<GameplayRequest>>>,
}

impl Resource for GameplayRequestInbox {}

pub fn gameplay_request_channel() -> (GameplayRequestSender, GameplayRequestInbox) {
    let (requests, inbox) = mpsc::channel();

    (
        GameplayRequestSender { requests },
        GameplayRequestInbox {
            requests: Arc::new(Mutex::new(inbox)),
        },
    )
}

impl GameplayRequestSender {
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
