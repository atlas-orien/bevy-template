use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::BoneId3d;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SkeletonAsset3d {
    pub bones: Vec<BoneDefinition3d>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BoneDefinition3d {
    pub id: BoneId3d,
    pub name: String,
    pub parent: Option<BoneParent3d>,
    pub rest_pose: BoneRestPose3d,
    pub inverse_bind_pose: Option<BoneInverseBindPose3d>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize, Serialize)]
pub struct BoneParent3d {
    pub bone: BoneId3d,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct BoneRestPose3d {
    pub translation: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct BoneInverseBindPose3d {
    pub matrix: [f32; 16],
}

impl SkeletonAsset3d {
    pub fn new(bones: Vec<BoneDefinition3d>) -> Self {
        Self { bones }
    }

    pub fn bone(&self, id: BoneId3d) -> Option<&BoneDefinition3d> {
        self.bones.iter().find(|bone| bone.id == id)
    }
}

impl BoneDefinition3d {
    pub fn root(id: BoneId3d, name: impl Into<String>, rest_pose: BoneRestPose3d) -> Self {
        Self {
            id,
            name: name.into(),
            parent: None,
            rest_pose,
            inverse_bind_pose: None,
        }
    }

    pub fn child(
        id: BoneId3d,
        name: impl Into<String>,
        parent: BoneId3d,
        rest_pose: BoneRestPose3d,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            parent: Some(BoneParent3d { bone: parent }),
            rest_pose,
            inverse_bind_pose: None,
        }
    }

    pub fn with_inverse_bind_pose(mut self, inverse_bind_pose: BoneInverseBindPose3d) -> Self {
        self.inverse_bind_pose = Some(inverse_bind_pose);
        self
    }
}

impl BoneRestPose3d {
    pub const fn new(translation: [f32; 3], rotation: [f32; 4], scale: [f32; 3]) -> Self {
        Self {
            translation,
            rotation,
            scale,
        }
    }

    pub const fn identity() -> Self {
        Self {
            translation: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0, 1.0, 1.0],
        }
    }

    pub fn into_transform(self) -> Transform {
        Transform {
            translation: Vec3::from_array(self.translation),
            rotation: Quat::from_array(self.rotation),
            scale: Vec3::from_array(self.scale),
        }
    }
}

impl From<Transform> for BoneRestPose3d {
    fn from(transform: Transform) -> Self {
        Self {
            translation: transform.translation.to_array(),
            rotation: transform.rotation.to_array(),
            scale: transform.scale.to_array(),
        }
    }
}

impl From<BoneRestPose3d> for Transform {
    fn from(rest_pose: BoneRestPose3d) -> Self {
        rest_pose.into_transform()
    }
}

impl BoneInverseBindPose3d {
    pub const fn new(matrix: [f32; 16]) -> Self {
        Self { matrix }
    }

    pub const fn identity() -> Self {
        Self {
            matrix: [
                1.0, 0.0, 0.0, 0.0, //
                0.0, 1.0, 0.0, 0.0, //
                0.0, 0.0, 1.0, 0.0, //
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
}
