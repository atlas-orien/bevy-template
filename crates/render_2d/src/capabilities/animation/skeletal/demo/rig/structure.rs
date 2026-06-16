use bevy::prelude::*;

use super::layout::{DemoJoint2d, DemoSkeletonSide};
use super::parts::{DemoSkeletonArm2d, DemoSkeletonShoulder2d, DemoSkeletonTorso2d};

pub struct DemoSkeleton2dRig {
    torso: DemoSkeletonTorso2d,
    left_shoulder: DemoSkeletonShoulder2d,
    right_shoulder: DemoSkeletonShoulder2d,
    left_arm: DemoSkeletonArm2d,
    right_arm: DemoSkeletonArm2d,
}

impl DemoSkeleton2dRig {
    pub(in crate::capabilities::animation::skeletal::demo) fn new(
        bone_image: Handle<Image>,
        joint_image: Handle<Image>,
    ) -> Self {
        Self {
            torso: DemoSkeletonTorso2d::new(bone_image.clone()),
            left_shoulder: DemoSkeletonShoulder2d::new(
                DemoJoint2d::LeftShoulder,
                joint_image.clone(),
            ),
            right_shoulder: DemoSkeletonShoulder2d::new(
                DemoJoint2d::RightShoulder,
                joint_image.clone(),
            ),
            left_arm: DemoSkeletonArm2d::new(
                DemoSkeletonSide::Left,
                bone_image.clone(),
                joint_image.clone(),
            ),
            right_arm: DemoSkeletonArm2d::new(DemoSkeletonSide::Right, bone_image, joint_image),
        }
    }

    pub(in crate::capabilities::animation::skeletal::demo) fn into_children(self) -> impl Bundle {
        children![
            self.torso.into_bundle(),
            self.left_shoulder.into_bundle(),
            self.right_shoulder.into_bundle(),
            self.left_arm.into_bundle(),
            self.right_arm.into_bundle(),
        ]
    }
}
