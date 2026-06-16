use bevy::prelude::*;

use super::rig::{DemoSkeleton2dBundle, DemoSkeleton2dRig};

pub struct DemoSkeleton2d {
    pub bundle: DemoSkeleton2dBundle,
    pub rig: DemoSkeleton2dRig,
}

impl DemoSkeleton2d {
    pub fn new(translation: Vec3, bone_image: Handle<Image>, joint_image: Handle<Image>) -> Self {
        Self {
            bundle: DemoSkeleton2dBundle::new(translation),
            rig: DemoSkeleton2dRig::new(bone_image, joint_image),
        }
    }

    pub fn into_bundle(self) -> impl Bundle {
        (self.bundle, self.rig.into_children())
    }
}
