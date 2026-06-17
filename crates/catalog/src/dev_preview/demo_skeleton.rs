use bevy::prelude::*;
use prefab::world_2d::demo_level::DemoSkeletonPrefab;

use crate::paths::{DEMO_SKELETON_BONE_IMAGE, DEMO_SKELETON_JOINT_IMAGE};

pub struct DemoSkeleton {
    position: Vec2,
}

impl DemoSkeleton {
    pub fn at(position: Vec2) -> Self {
        Self { position }
    }

    pub fn prefab(self, asset_server: &AssetServer) -> DemoSkeletonPrefab {
        DemoSkeletonPrefab::new(
            self.position,
            asset_server.load(DEMO_SKELETON_BONE_IMAGE),
            asset_server.load(DEMO_SKELETON_JOINT_IMAGE),
        )
    }
}
