use bevy::prelude::*;
use prefab::world_2d::demo_level::DemoSkeletonPrefab;

const DEMO_SKELETON_BONE_IMAGE: &str = "2d/static/props/demo-skeletal-bone/demo-skeletal-bone.png";
const DEMO_SKELETON_JOINT_IMAGE: &str =
    "2d/static/props/demo-skeletal-joint/demo-skeletal-joint.png";

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
