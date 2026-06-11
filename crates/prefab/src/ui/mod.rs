//! Screen UI prefab namespace.
//!
//! UI prefabs are shared by 2D and 3D games because they live in screen space.

pub mod menu;

pub use menu::{DEMO_OPTIONS_ACTION, DEMO_QUIT_ACTION, DEMO_START_ACTION, DemoMenuPrefab};
