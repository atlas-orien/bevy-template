//! ECS 事件数据。
//!
//! 这里放系统之间传递的事件类型，例如伤害、治疗、死亡、拾取。
//! 事件只描述“发生了什么”，不处理事件带来的后果。

pub mod combat;
pub mod demo_sensor;
pub mod demo_session;
pub mod lifecycle;

use bevy::prelude::*;

use self::combat::{DamageEvent, HealEvent};
use self::demo_sensor::DemoSensorTriggeredEvent;
use self::demo_session::DemoSessionStartedEvent;
use self::lifecycle::{DiedEvent, SpawnedEvent};

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<DamageEvent>()
            .add_message::<HealEvent>()
            .add_message::<DemoSensorTriggeredEvent>()
            .add_message::<DemoSessionStartedEvent>()
            .add_message::<SpawnedEvent>()
            .add_message::<DiedEvent>();
    }
}
