use bevy::prelude::*;

use super::bundles::{DemoBone2dBundle, DemoJoint2dBundle};
use super::layout::{DemoJoint2d, DemoSkeletonSide};

pub(in crate::animation::skeletal::demo) struct DemoSkeletonTorso2d {
    image: Handle<Image>,
}

impl DemoSkeletonTorso2d {
    pub(in crate::animation::skeletal::demo) fn new(image: Handle<Image>) -> Self {
        Self { image }
    }

    pub(in crate::animation::skeletal::demo) fn into_bundle(self) -> DemoBone2dBundle {
        DemoBone2dBundle::torso(self.image)
    }
}

pub(in crate::animation::skeletal::demo) struct DemoSkeletonShoulder2d {
    joint: DemoJoint2d,
    image: Handle<Image>,
}

impl DemoSkeletonShoulder2d {
    pub(in crate::animation::skeletal::demo) fn new(
        joint: DemoJoint2d,
        image: Handle<Image>,
    ) -> Self {
        Self { joint, image }
    }

    pub(in crate::animation::skeletal::demo) fn into_bundle(self) -> DemoJoint2dBundle {
        DemoJoint2dBundle::new(self.image, self.joint, self.joint.translation())
    }
}

pub(in crate::animation::skeletal::demo) struct DemoSkeletonArm2d {
    side: DemoSkeletonSide,
    bone_image: Handle<Image>,
    joint_image: Handle<Image>,
}

impl DemoSkeletonArm2d {
    pub(in crate::animation::skeletal::demo) fn new(
        side: DemoSkeletonSide,
        bone_image: Handle<Image>,
        joint_image: Handle<Image>,
    ) -> Self {
        Self {
            side,
            bone_image,
            joint_image,
        }
    }

    pub(in crate::animation::skeletal::demo) fn into_bundle(self) -> impl Bundle {
        (
            DemoBone2dBundle::upper_arm(
                self.bone_image.clone(),
                self.side.upper_arm_bone(),
                self.side,
            ),
            children![
                DemoJoint2dBundle::new(
                    self.joint_image,
                    self.side.elbow_joint(),
                    self.side.elbow_joint().translation(),
                ),
                DemoBone2dBundle::lower_arm(self.bone_image, self.side.lower_arm_bone(), self.side,),
            ],
        )
    }
}
