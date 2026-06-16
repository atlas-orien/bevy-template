use bevy::prelude::*;

use super::rig::{DemoSkeleton2dChildrenBundle, DemoSkeleton2dRig, DemoSkeleton2dRootBundle};

pub struct DemoSkeleton2d {
    pub root: DemoSkeleton2dRootBundle,
    pub rig: DemoSkeleton2dRig,
}

#[derive(Bundle)]
#[bundle(ignore_from_components)]
pub struct DemoSkeleton2dBundle {
    root: DemoSkeleton2dRootBundle,
    children: DemoSkeleton2dChildrenBundle,
}

impl DemoSkeleton2d {
    pub fn new(translation: Vec3, bone_image: Handle<Image>, joint_image: Handle<Image>) -> Self {
        Self {
            root: DemoSkeleton2dRootBundle::new(translation),
            rig: DemoSkeleton2dRig::new(bone_image, joint_image),
        }
    }

    pub fn into_bundle(self) -> DemoSkeleton2dBundle {
        DemoSkeleton2dBundle {
            root: self.root,
            children: self.rig.into_children(),
        }
    }
}

impl DemoSkeleton2dBundle {
    pub fn new(translation: Vec3, bone_image: Handle<Image>, joint_image: Handle<Image>) -> Self {
        DemoSkeleton2d::new(translation, bone_image, joint_image).into_bundle()
    }
}
