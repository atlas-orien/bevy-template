//! Demo 2D 骨骼动画产品，使用 Bevy 原生 Transform 层级表达骨骼。

mod entry;
mod rig;
mod systems;

pub use entry::DemoSkeleton2d;
pub use rig::{DemoSkeleton2dBundle, DemoSkeleton2dRig};
pub(super) use systems::DemoSkeletalAnimationPlugin;

#[cfg(test)]
mod tests;
