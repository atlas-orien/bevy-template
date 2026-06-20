use bevy::prelude::*;

use crate::primitives::skeletons::SkeletonId3d;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct RigBinding3d {
    pub skeleton: SkeletonId3d,
}

impl RigBinding3d {
    pub const fn new(skeleton: SkeletonId3d) -> Self {
        Self { skeleton }
    }
}
