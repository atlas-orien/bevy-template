//! 把 physics 传感器事件过滤转发为 demo 感应区事件。

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bridge_forwards_events_from_demo_sensor_zones() {
        let mut app = App::new();
        app.add_message::<PhysicsSensorTriggered>()
            .add_message::<DemoSensorTriggeredEvent>()
            .add_systems(Update, demo_sensor_bridge_system);
        let sensor = app.world_mut().spawn(DemoSensorZone).id();
        let target = app.world_mut().spawn_empty().id();
        app.world_mut()
            .write_message(PhysicsSensorTriggered { sensor, target });

        app.update();

        let events = app.world().resource::<Messages<DemoSensorTriggeredEvent>>();
        let forwarded: Vec<_> = events.iter_current_update_messages().copied().collect();
        assert_eq!(forwarded, vec![DemoSensorTriggeredEvent { sensor, target }]);
    }

    #[test]
    fn bridge_filters_events_from_unmarked_sensors() {
        let mut app = App::new();
        app.add_message::<PhysicsSensorTriggered>()
            .add_message::<DemoSensorTriggeredEvent>()
            .add_systems(Update, demo_sensor_bridge_system);
        let sensor = app.world_mut().spawn_empty().id();
        let target = app.world_mut().spawn_empty().id();
        app.world_mut()
            .write_message(PhysicsSensorTriggered { sensor, target });

        app.update();

        let events = app.world().resource::<Messages<DemoSensorTriggeredEvent>>();
        assert_eq!(events.iter_current_update_messages().count(), 0);
    }
}
