//! 跟随目标的 2D 场景相机。

mod entry;
mod systems;

pub use entry::{FollowCamera2d, FollowCamera2dBundle, FollowCameraTarget2d};
pub(super) use systems::follow_camera_system;
