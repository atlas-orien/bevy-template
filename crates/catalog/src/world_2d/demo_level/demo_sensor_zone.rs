use bevy::prelude::*;
use prefab::world_2d::demo_level::DemoSensorZonePrefab;

const DEMO_SENSOR_AUDIO: &str = "audio/demo_pickup.ogg";

pub struct DemoSensorZone {
    position: Vec2,
}

impl DemoSensorZone {
    pub fn at(position: Vec2) -> Self {
        Self { position }
    }

    pub fn prefab(self) -> DemoSensorZonePrefab {
        DemoSensorZonePrefab::new(self.position, DEMO_SENSOR_AUDIO)
    }
}
