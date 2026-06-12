use bevy::prelude::*;
use ecs::components::world::DemoSensorZone;
use ecs::events::demo_sensor::DemoSensorTriggeredEvent;
use physics::PhysicsSensorTriggered;

pub fn demo_sensor_bridge_system(
    mut sensor_events: MessageReader<PhysicsSensorTriggered>,
    sensors: Query<(), With<DemoSensorZone>>,
    mut demo_events: MessageWriter<DemoSensorTriggeredEvent>,
) {
    for event in sensor_events.read() {
        if sensors.get(event.sensor).is_err() {
            continue;
        }

        demo_events.write(DemoSensorTriggeredEvent {
            sensor: event.sensor,
            target: event.target,
        });
    }
}
