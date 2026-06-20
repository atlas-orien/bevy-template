mod plugin;
mod pose;
mod rig;
mod skeleton;
mod systems;

pub use plugin::Skeleton3dPlugin;
pub use pose::{BonePose3d, Pose3d};
pub use rig::RigBinding3d;
pub use skeleton::{Bone3d, BoneId3d, Skeleton3d, SkeletonId3d};

#[cfg(test)]
mod tests;
