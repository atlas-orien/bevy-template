//! Demo gameplay session 开始事件。

use bevy::prelude::*;

#[derive(Message, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct DemoSessionStartedEvent;
