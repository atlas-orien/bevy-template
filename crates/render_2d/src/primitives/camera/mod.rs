mod base;
mod plugin;
pub mod presets;

pub use plugin::Camera2dPlugin;
pub use presets::{
    FixedCamera2dBundle, FollowCamera2d, FollowCamera2dBundle, FollowCameraTarget2dMarker,
    SceneCamera2dMarker, UI_CAMERA_ORDER, UiCamera,
};
