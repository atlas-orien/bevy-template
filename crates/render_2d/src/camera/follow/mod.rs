//! 跟随目标的 2D 场景相机。

mod entry;
mod systems;

pub use entry::{FollowCamera2d, FollowCamera2dBundle};
pub(super) use systems::follow_camera_system;
