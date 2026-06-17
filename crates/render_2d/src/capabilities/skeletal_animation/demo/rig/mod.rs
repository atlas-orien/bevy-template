mod bundles;
mod layout;
mod parts;
mod structure;

#[cfg(test)]
pub(super) use bundles::DemoBone2dBundle;
pub use bundles::DemoSkeleton2dRootBundle;
pub(super) use bundles::{DemoBone2d, DemoSkeleton2dRootMarker};
pub(super) use layout::DemoSkeletonSide;
pub use structure::{DemoSkeleton2dChildrenBundle, DemoSkeleton2dRig};
