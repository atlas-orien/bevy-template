pub mod demo_props;
pub mod example;

use bevy::prelude::*;

pub use demo_props::{
    DemoLandmark2d, DemoLandmark2dBundle, DemoRock2d, DemoRock2dBundle, DemoSensorZone2d,
    DemoSensorZone2dBundle,
};

pub struct PropsPlugin;

impl Plugin for PropsPlugin {
    fn build(&self, _app: &mut App) {}
}
