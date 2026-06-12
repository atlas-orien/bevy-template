use bevy::prelude::*;
use prefab::demo_events::DemoSensorTriggeredEvent;

pub fn handle_demo_sensor_triggered_system(mut events: MessageReader<DemoSensorTriggeredEvent>) {
    for event in events.read() {
        info!(
            "demo sensor triggered: sensor={:?}, target={:?}",
            event.sensor, event.target
        );
    }
}
