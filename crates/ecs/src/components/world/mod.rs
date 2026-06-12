pub mod demo_sensor;
pub mod gameplay;

use bevy::prelude::*;

pub use demo_sensor::DemoSensorZone;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, _app: &mut App) {}
}
