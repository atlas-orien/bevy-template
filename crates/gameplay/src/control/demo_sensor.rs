use bevy::prelude::*;
use prefab::control::LocallyControlledQuery;
use prefab::demo_events::DemoSensorTriggeredEvent;
use prefab::health::{HealthQuery, damage_entity};

use crate::state::AppState;

pub fn handle_demo_sensor_triggered_system(
    mut events: MessageReader<DemoSensorTriggeredEvent>,
    controlled: LocallyControlledQuery,
    mut health: HealthQuery,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for event in events.read() {
        info!(
            "demo sensor triggered: sensor={:?}, target={:?}",
            event.sensor, event.target
        );

        if controlled.get(event.target).is_err() {
            continue;
        }

        let Some(health) = damage_entity(event.target, 10.0, &mut health) else {
            continue;
        };
        if health.is_empty() {
            next_state.set(AppState::GameOver);
        }
    }
}
