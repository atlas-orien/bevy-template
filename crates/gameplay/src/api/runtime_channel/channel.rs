use bevy::prelude::*;

use helper::channel::{self, ChannelInbox, ChannelSender};

use super::message::{RuntimeRequest, RuntimeUpdate};

pub type RuntimeRequestSender = ChannelSender<RuntimeRequest>;
pub type RuntimeRequestInbox = ChannelInbox<RuntimeRequest>;
pub type RuntimeUpdateSender = ChannelSender<RuntimeUpdate>;
pub type RuntimeUpdateInbox = ChannelInbox<RuntimeUpdate>;

#[derive(Clone)]
pub struct RuntimeRequestChannel {
    sender: RuntimeRequestSender,
    inbox: RuntimeRequestInbox,
}

#[derive(Clone)]
pub struct ManagerUpdateChannel {
    sender: RuntimeUpdateSender,
    inbox: RuntimeUpdateInbox,
}

impl RuntimeRequestChannel {
    pub fn new() -> Self {
        let (sender, inbox) = channel::channel();
        Self { sender, inbox }
    }

    pub fn sender(&self) -> RuntimeRequestSender {
        self.sender.clone()
    }

    pub fn inbox(&self) -> RuntimeRequestInbox {
        self.inbox.clone()
    }
}

impl Default for RuntimeRequestChannel {
    fn default() -> Self {
        Self::new()
    }
}

impl ManagerUpdateChannel {
    pub fn new() -> Self {
        let (sender, inbox) = channel::channel();
        Self { sender, inbox }
    }

    pub fn sender(&self) -> RuntimeUpdateSender {
        self.sender.clone()
    }

    pub fn inbox(&self) -> RuntimeUpdateInbox {
        self.inbox.clone()
    }
}

impl Default for ManagerUpdateChannel {
    fn default() -> Self {
        Self::new()
    }
}

pub fn drain_runtime_requests_into(
    inbox: &RuntimeRequestInbox,
    requests: &mut MessageWriter<RuntimeRequest>,
) {
    for request in inbox.drain() {
        requests.write(request);
    }
}
