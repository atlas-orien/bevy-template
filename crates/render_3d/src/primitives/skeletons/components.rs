use bevy::prelude::*;

use super::{BoneId3d, SkeletonId3d};

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct Skeleton3d {
    pub id: SkeletonId3d,
}

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct Bone3d {
    pub skeleton: SkeletonId3d,
    pub id: BoneId3d,
}

impl Skeleton3d {
    pub const fn new(id: SkeletonId3d) -> Self {
        Self { id }
    }
}

impl Bone3d {
    pub const fn new(skeleton: SkeletonId3d, id: BoneId3d) -> Self {
        Self { skeleton, id }
    }
}
