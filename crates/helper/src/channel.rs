use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};

use bevy::prelude::Resource;

pub struct ChannelSender<T> {
    inner: Sender<T>,
}

pub struct ChannelInbox<T> {
    inner: Arc<Mutex<Receiver<T>>>,
}

impl<T> Clone for ChannelSender<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Clone for ChannelInbox<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: Send + 'static> Resource for ChannelSender<T> {}
impl<T: Send + 'static> Resource for ChannelInbox<T> {}

pub fn channel<T>() -> (ChannelSender<T>, ChannelInbox<T>) {
    let (sender, receiver) = mpsc::channel();

    (
        ChannelSender { inner: sender },
        ChannelInbox {
            inner: Arc::new(Mutex::new(receiver)),
        },
    )
}

impl<T> ChannelSender<T> {
    pub fn send(&self, message: T) -> bool {
        self.inner.send(message).is_ok()
    }
}

impl<T> ChannelInbox<T> {
    pub fn drain(&self) -> Vec<T> {
        let Ok(inbox) = self.inner.lock() else {
            return Vec::new();
        };

        inbox.try_iter().collect()
    }
}
