//! Demo 感应区 sprite 表现。

use bevy::prelude::*;

const DEMO_SENSOR_ZONE_COLOR: Color = Color::srgba(0.2, 0.75, 0.95, 0.38);
pub const DEMO_SENSOR_ZONE_SIZE: Vec2 = Vec2::new(72.0, 44.0);

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
struct DemoSensorZone2dMarker;

#[derive(Bundle)]
pub struct DemoSensorZone2d {
    marker: DemoSensorZone2dMarker,
    sprite: Sprite,
}

impl Default for DemoSensorZone2d {
    fn default() -> Self {
        Self {
            marker: DemoSensorZone2dMarker,
            sprite: Sprite {
                color: DEMO_SENSOR_ZONE_COLOR,
                custom_size: Some(DEMO_SENSOR_ZONE_SIZE),
                ..default()
            },
        }
    }
}
