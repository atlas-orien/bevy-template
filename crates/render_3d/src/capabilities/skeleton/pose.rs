use bevy::prelude::*;

use super::{BoneId3d, SkeletonId3d};

#[derive(Component, Debug, Clone, Copy)]
pub struct BonePose3d {
    pub skeleton: SkeletonId3d,
    pub bone: BoneId3d,
    pub local_transform: Transform,
}

#[derive(Component, Debug, Clone)]
pub struct Pose3d {
    pub skeleton: SkeletonId3d,
    pub bones: Vec<BonePose3d>,
}

impl BonePose3d {
    pub const fn new(skeleton: SkeletonId3d, bone: BoneId3d, local_transform: Transform) -> Self {
        Self {
            skeleton,
            bone,
            local_transform,
        }
    }
}

impl Pose3d {
    pub fn new(skeleton: SkeletonId3d, bones: Vec<BonePose3d>) -> Self {
        Self { skeleton, bones }
    }

    pub fn empty(skeleton: SkeletonId3d) -> Self {
        Self {
            skeleton,
            bones: Vec::new(),
        }
    }

    pub fn with_bone(mut self, bone: BoneId3d, local_transform: Transform) -> Self {
        self.bones
            .push(BonePose3d::new(self.skeleton, bone, local_transform));
        self
    }
}
