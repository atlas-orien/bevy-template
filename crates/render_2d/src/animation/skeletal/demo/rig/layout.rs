use bevy::prelude::*;

use super::bundles::DemoBone2d;

pub(in crate::animation::skeletal::demo) const DEMO_TORSO_SIZE: Vec2 = Vec2::new(18.0, 42.0);
pub(in crate::animation::skeletal::demo) const DEMO_ARM_SIZE: Vec2 = Vec2::new(8.0, 30.0);
pub(in crate::animation::skeletal::demo) const DEMO_JOINT_SIZE: Vec2 = Vec2::splat(13.0);
pub(in crate::animation::skeletal::demo) const DEMO_BONE_COLOR: Color =
    Color::srgb(0.38, 0.84, 0.95);
pub(in crate::animation::skeletal::demo) const DEMO_BONE_Z: f32 = 6.0;
pub(in crate::animation::skeletal::demo) const DEMO_JOINT_Z: f32 = 7.0;

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq)]
pub(in crate::animation::skeletal::demo) enum DemoJoint2d {
    LeftShoulder,
    LeftElbow,
    RightShoulder,
    RightElbow,
}

impl DemoJoint2d {
    pub(in crate::animation::skeletal::demo) fn translation(self) -> Vec3 {
        match self {
            Self::LeftShoulder => Vec3::new(-15.0, 34.0, 0.0),
            Self::RightShoulder => Vec3::new(15.0, 34.0, 0.0),
            Self::LeftElbow | Self::RightElbow => Vec3::new(0.0, -26.0, 0.0),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(in crate::animation::skeletal::demo) enum DemoSkeletonSide {
    Left,
    Right,
}

impl DemoSkeletonSide {
    pub(in crate::animation::skeletal::demo) fn x_sign(self) -> f32 {
        match self {
            Self::Left => -1.0,
            Self::Right => 1.0,
        }
    }

    pub(in crate::animation::skeletal::demo) fn rest_angle(self) -> f32 {
        self.x_sign() * 0.28
    }

    pub(in crate::animation::skeletal::demo) fn mirrored_swing(self, swing: f32) -> f32 {
        match self {
            Self::Left => swing,
            Self::Right => -swing,
        }
    }

    pub(in crate::animation::skeletal::demo) fn upper_arm_bone(self) -> DemoBone2d {
        match self {
            Self::Left => DemoBone2d::LeftUpperArm,
            Self::Right => DemoBone2d::RightUpperArm,
        }
    }

    pub(in crate::animation::skeletal::demo) fn lower_arm_bone(self) -> DemoBone2d {
        match self {
            Self::Left => DemoBone2d::LeftLowerArm,
            Self::Right => DemoBone2d::RightLowerArm,
        }
    }

    pub(in crate::animation::skeletal::demo) fn elbow_joint(self) -> DemoJoint2d {
        match self {
            Self::Left => DemoJoint2d::LeftElbow,
            Self::Right => DemoJoint2d::RightElbow,
        }
    }
}
