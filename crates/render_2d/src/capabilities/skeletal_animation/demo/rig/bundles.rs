use bevy::prelude::*;
use bevy::sprite::Anchor;

use super::layout::{
    DEMO_ARM_SIZE, DEMO_BONE_COLOR, DEMO_BONE_Z, DEMO_JOINT_SIZE, DEMO_JOINT_Z, DEMO_TORSO_SIZE,
    DemoJoint2d, DemoSkeletonSide,
};
use crate::capabilities::skeletal_animation::demo::systems::DemoSkeletalAnimation2d;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub(in crate::capabilities::skeletal_animation::demo) struct DemoSkeleton2dRoot;

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq)]
pub(in crate::capabilities::skeletal_animation::demo) enum DemoBone2d {
    Torso,
    LeftUpperArm,
    LeftLowerArm,
    RightUpperArm,
    RightLowerArm,
}

#[derive(Bundle)]
pub struct DemoSkeleton2dRootBundle {
    marker: DemoSkeleton2dRoot,
    animation: DemoSkeletalAnimation2d,
    transform: Transform,
    visibility: Visibility,
}

impl DemoSkeleton2dRootBundle {
    pub(in crate::capabilities::skeletal_animation::demo) fn new(translation: Vec3) -> Self {
        Self {
            marker: DemoSkeleton2dRoot,
            animation: DemoSkeletalAnimation2d::default(),
            transform: Transform::from_translation(translation),
            visibility: Visibility::default(),
        }
    }
}

#[derive(Bundle)]
pub(in crate::capabilities::skeletal_animation::demo) struct DemoBone2dBundle {
    bone: DemoBone2d,
    sprite: Sprite,
    anchor: Anchor,
    transform: Transform,
}

impl DemoBone2dBundle {
    pub(in crate::capabilities::skeletal_animation::demo) fn torso(image: Handle<Image>) -> Self {
        Self {
            bone: DemoBone2d::Torso,
            sprite: bone_sprite(image, DEMO_TORSO_SIZE),
            anchor: Anchor::TOP_CENTER,
            transform: Transform::from_translation(Vec3::new(0.0, 18.0, DEMO_BONE_Z)),
        }
    }

    pub(in crate::capabilities::skeletal_animation::demo) fn upper_arm(
        image: Handle<Image>,
        bone: DemoBone2d,
        side: DemoSkeletonSide,
    ) -> Self {
        Self {
            bone,
            sprite: bone_sprite(image, DEMO_ARM_SIZE),
            anchor: Anchor::TOP_CENTER,
            transform: Transform {
                translation: Vec3::new(side.x_sign() * 15.0, 34.0, DEMO_BONE_Z),
                rotation: Quat::from_rotation_z(side.rest_angle()),
                ..default()
            },
        }
    }

    pub(in crate::capabilities::skeletal_animation::demo) fn lower_arm(
        image: Handle<Image>,
        bone: DemoBone2d,
        side: DemoSkeletonSide,
    ) -> Self {
        Self {
            bone,
            sprite: bone_sprite(image, DEMO_ARM_SIZE),
            anchor: Anchor::TOP_CENTER,
            transform: Transform {
                translation: Vec3::new(0.0, -26.0, 0.0),
                rotation: Quat::from_rotation_z(side.rest_angle() * 0.35),
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub(in crate::capabilities::skeletal_animation::demo) struct DemoJoint2dBundle {
    joint: DemoJoint2d,
    sprite: Sprite,
    transform: Transform,
}

impl DemoJoint2dBundle {
    pub(in crate::capabilities::skeletal_animation::demo) fn new(
        image: Handle<Image>,
        joint: DemoJoint2d,
        translation: Vec3,
    ) -> Self {
        Self {
            joint,
            sprite: Sprite {
                image,
                custom_size: Some(DEMO_JOINT_SIZE),
                ..default()
            },
            transform: Transform::from_translation(translation.with_z(DEMO_JOINT_Z)),
        }
    }
}

fn bone_sprite(image: Handle<Image>, size: Vec2) -> Sprite {
    Sprite {
        image,
        color: DEMO_BONE_COLOR,
        custom_size: Some(size),
        ..default()
    }
}
