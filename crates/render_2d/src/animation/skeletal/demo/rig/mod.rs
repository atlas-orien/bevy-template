mod bundles;
mod layout;
mod parts;
mod structure;

#[cfg(test)]
pub(super) use bundles::DemoBone2dBundle;
pub use bundles::DemoSkeleton2dBundle;
pub(super) use bundles::{DemoBone2d, DemoSkeleton2dRoot};
pub(super) use layout::DemoSkeletonSide;
pub use structure::DemoSkeleton2dRig;
