use bevy::prelude::*;

use super::combat::{DamageEvent, HealEvent};
use super::demo_sensor::DemoSensorTriggeredEvent;
use super::demo_session::DemoSessionStartedEvent;
use super::lifecycle::{DiedEvent, SpawnedEvent};

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
