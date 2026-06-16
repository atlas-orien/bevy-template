use bevy::prelude::*;

use super::bundles::{DemoBone2dBundle, DemoJoint2dBundle};
use super::layout::{DemoJoint2d, DemoSkeletonSide};

pub(in crate::capabilities::skeletal_animation::demo) struct DemoSkeletonTorso2d {
    image: Handle<Image>,
}

impl DemoSkeletonTorso2d {
    pub(in crate::capabilities::skeletal_animation::demo) fn new(image: Handle<Image>) -> Self {
        Self { image }
    }

    pub(in crate::capabilities::skeletal_animation::demo) fn into_bundle(self) -> DemoBone2dBundle {
        DemoBone2dBundle::torso(self.image)
    }
}

pub(in crate::capabilities::skeletal_animation::demo) struct DemoSkeletonShoulder2d {
    joint: DemoJoint2d,
    image: Handle<Image>,
}

impl DemoSkeletonShoulder2d {
    pub(in crate::capabilities::skeletal_animation::demo) fn new(
        joint: DemoJoint2d,
        image: Handle<Image>,
    ) -> Self {
        Self { joint, image }
    }

    pub(in crate::capabilities::skeletal_animation::demo) fn into_bundle(
        self,
    ) -> DemoJoint2dBundle {
        DemoJoint2dBundle::new(self.image, self.joint, self.joint.translation())
    }
}

pub(in crate::capabilities::skeletal_animation::demo) struct DemoSkeletonArm2d {
    side: DemoSkeletonSide,
    bone_image: Handle<Image>,
    joint_image: Handle<Image>,
}

#[derive(Bundle)]
#[bundle(ignore_from_components)]
pub(in crate::capabilities::skeletal_animation::demo) struct DemoSkeletonArm2dBundle(
    DemoBone2dBundle,
    bevy::ecs::spawn::SpawnRelatedBundle<
        bevy::ecs::hierarchy::ChildOf,
        (
            bevy::ecs::spawn::Spawn<DemoJoint2dBundle>,
            bevy::ecs::spawn::Spawn<DemoBone2dBundle>,
        ),
    >,
);

impl DemoSkeletonArm2d {
    pub(in crate::capabilities::skeletal_animation::demo) fn new(
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

    pub(in crate::capabilities::skeletal_animation::demo) fn into_bundle(
        self,
    ) -> DemoSkeletonArm2dBundle {
        DemoSkeletonArm2dBundle(
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
