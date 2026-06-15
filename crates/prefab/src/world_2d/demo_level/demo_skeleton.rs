//! Demo 2D 骨骼展示 prefab。

use bevy::prelude::*;
use ecs::components::world::gameplay::GameplaySessionEntity;
use render_2d::animation::skeletal::{
    DemoBone2d, DemoBone2dBundle, DemoJoint2d, DemoJoint2dBundle, DemoSkeleton2dBundle,
    DemoSkeletonSide,
};

use crate::Prefab;

const DEMO_SKELETON_Z: f32 = 3.0;

pub struct DemoSkeletonPrefab {
    position: Vec2,
    bone_image: Handle<Image>,
    joint_image: Handle<Image>,
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
            .spawn((
                GameplaySessionEntity,
                DemoSkeleton2dBundle::new(Vec3::new(
                    self.position.x,
                    self.position.y,
                    DEMO_SKELETON_Z,
                )),
                children![
                    DemoBone2dBundle::torso(self.bone_image.clone()),
                    DemoJoint2dBundle::new(
                        self.joint_image.clone(),
                        DemoJoint2d::LeftShoulder,
                        Vec3::new(-15.0, 34.0, 0.0),
                    ),
                    DemoJoint2dBundle::new(
                        self.joint_image.clone(),
                        DemoJoint2d::RightShoulder,
                        Vec3::new(15.0, 34.0, 0.0),
                    ),
                    (
                        DemoBone2dBundle::upper_arm(
                            self.bone_image.clone(),
                            DemoBone2d::LeftUpperArm,
                            DemoSkeletonSide::Left,
                        ),
                        children![
                            DemoJoint2dBundle::new(
                                self.joint_image.clone(),
                                DemoJoint2d::LeftElbow,
                                Vec3::new(0.0, -26.0, 0.0),
                            ),
                            DemoBone2dBundle::lower_arm(
                                self.bone_image.clone(),
                                DemoBone2d::LeftLowerArm,
                                DemoSkeletonSide::Left,
                            ),
                        ],
                    ),
                    (
                        DemoBone2dBundle::upper_arm(
                            self.bone_image.clone(),
                            DemoBone2d::RightUpperArm,
                            DemoSkeletonSide::Right,
                        ),
                        children![
                            DemoJoint2dBundle::new(
                                self.joint_image.clone(),
                                DemoJoint2d::RightElbow,
                                Vec3::new(0.0, -26.0, 0.0),
                            ),
                            DemoBone2dBundle::lower_arm(
                                self.bone_image.clone(),
                                DemoBone2d::RightLowerArm,
                                DemoSkeletonSide::Right,
                            ),
                        ],
                    ),
                ],
            ))
            .id()
    }
}
