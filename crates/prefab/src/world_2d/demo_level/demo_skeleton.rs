//! Demo 2D 骨骼展示 prefab。

use bevy::prelude::*;
use ecs::components::world::gameplay::GameplaySessionEntityMarker;
use render_2d::capabilities::skeletal_animation::DemoSkeleton2dBundle;

use crate::Prefab;

const DEMO_SKELETON_Z: f32 = 3.0;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
struct DemoSkeletonRootMarker;

#[derive(Bundle)]
#[bundle(ignore_from_components)]
pub struct DemoSkeletonPrefab {
    root: DemoSkeletonRootMarker,
    session: GameplaySessionEntityMarker,
    visual: DemoSkeleton2dBundle,
}

impl DemoSkeletonPrefab {
    pub fn new(position: Vec2, bone_image: Handle<Image>, joint_image: Handle<Image>) -> Self {
        Self {
            root: DemoSkeletonRootMarker,
            session: GameplaySessionEntityMarker,
            visual: DemoSkeleton2dBundle::new(
                Vec3::new(position.x, position.y, DEMO_SKELETON_Z),
                bone_image,
                joint_image,
            ),
        }
    }
}

impl Prefab for DemoSkeletonPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands.spawn(self).id()
    }
}
