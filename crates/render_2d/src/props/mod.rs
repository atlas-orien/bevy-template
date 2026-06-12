pub mod demo_props;
pub mod example;
mod plugin;

pub use demo_props::{
    DEMO_SENSOR_ZONE_SIZE, DemoLandmark2d, DemoLandmark2dBundle, DemoRock2d, DemoRock2dBundle,
    DemoSensorZone2d, DemoSensorZone2dBundle,
};
pub use plugin::PropsPlugin;
