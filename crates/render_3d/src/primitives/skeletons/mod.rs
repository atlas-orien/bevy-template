use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct SkeletonId3d(pub u64);

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct BoneId3d(pub u16);

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
