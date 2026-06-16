mod base;
mod markers;
mod plugin;
pub mod presets;

pub use markers::{FollowCameraTarget2d, SceneCamera2d};
pub use plugin::Camera2dPlugin;
pub use presets::{
    FixedCamera2dBundle, FollowCamera2d, FollowCamera2dBundle, UI_CAMERA_ORDER, UiCamera,
};
