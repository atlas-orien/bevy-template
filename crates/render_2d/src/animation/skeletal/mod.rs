pub mod demo_skeletal_animation;
mod plugin;

pub use demo_skeletal_animation::{
    DemoBone2d, DemoBone2dBundle, DemoJoint2d, DemoJoint2dBundle, DemoSkeletalAnimation2d,
    DemoSkeleton2d, DemoSkeleton2dBundle, DemoSkeletonSide, demo_bone_rotation,
    demo_skeletal_animation_system,
};
pub use plugin::SkeletalAnimation2dPlugin;
