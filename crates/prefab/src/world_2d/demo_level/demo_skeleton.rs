//! Demo 2D 骨骼展示 prefab。

use bevy::prelude::*;
use ecs::components::world::gameplay::GameplaySessionEntity;
use render_2d::capabilities::skeletal_animation::DemoSkeleton2d;

use crate::Prefab;

const DEMO_SKELETON_Z: f32 = 3.0;

pub struct DemoSkeletonPrefab {
    position: Vec2,
    bone_image: Handle<Image>,
    joint_image: Handle<Image>,
}

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
struct DemoSkeletonRoot;

#[derive(Bundle, Default)]
struct DemoSkeletonRootBundle {
    root: DemoSkeletonRoot,
    session: GameplaySessionEntity,
}

impl DemoSkeletonPrefab {
    pub fn new(position: Vec2, bone_image: Handle<Image>, joint_image: Handle<Image>) -> Self {
        Self {
            position,
            bone_image,
            joint_image,
        }
    }
}

impl Prefab for DemoSkeletonPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands
            .spawn(DemoSkeletonRootBundle::default())
            .insert(
                DemoSkeleton2d::new(
                    Vec3::new(self.position.x, self.position.y, DEMO_SKELETON_Z),
                    self.bone_image,
                    self.joint_image,
                )
                .into_bundle(),
            )
            .id()
    }
}
