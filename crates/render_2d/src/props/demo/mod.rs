//! Demo 静物和关卡交互物 2D 表现。

mod landmark;
mod rock;
mod sensor_zone;

pub use landmark::DemoLandmark2d;
pub use rock::DemoRock2d;
pub use sensor_zone::{DEMO_SENSOR_ZONE_SIZE, DemoSensorZone2d};
