use bevy::mesh::skinning::{SkinnedMesh, SkinnedMeshInverseBindposes};
use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct SkinAsset3d {
    inverse_bindposes: Handle<SkinnedMeshInverseBindposes>,
}

#[derive(Debug, Clone)]
pub struct SkinJoints3d {
    joints: Vec<Entity>,
}

#[derive(Debug, Clone)]
pub struct Skin3d {
    asset: SkinAsset3d,
    joints: SkinJoints3d,
}

impl SkinAsset3d {
    pub fn new(inverse_bindposes: Handle<SkinnedMeshInverseBindposes>) -> Self {
        Self { inverse_bindposes }
    }

    pub fn inverse_bindposes(&self) -> &Handle<SkinnedMeshInverseBindposes> {
        &self.inverse_bindposes
    }

    pub fn clone_handle(&self) -> Handle<SkinnedMeshInverseBindposes> {
        self.inverse_bindposes.clone()
    }

    pub fn into_handle(self) -> Handle<SkinnedMeshInverseBindposes> {
        self.inverse_bindposes
    }
}

impl SkinJoints3d {
    pub fn new(joints: Vec<Entity>) -> Self {
        Self { joints }
    }

    pub fn joints(&self) -> &[Entity] {
        &self.joints
    }

    pub fn into_joints(self) -> Vec<Entity> {
        self.joints
    }
}

impl Skin3d {
    pub fn new(
        inverse_bindposes: Handle<SkinnedMeshInverseBindposes>,
        joints: Vec<Entity>,
    ) -> Self {
        Self {
            asset: SkinAsset3d::new(inverse_bindposes),
            joints: SkinJoints3d::new(joints),
        }
    }

    pub fn from_parts(asset: SkinAsset3d, joints: SkinJoints3d) -> Self {
        Self { asset, joints }
    }

    pub fn asset(&self) -> &SkinAsset3d {
        &self.asset
    }

    pub fn joints(&self) -> &SkinJoints3d {
        &self.joints
    }

    pub fn into_bundle(self) -> Skin3dBundle {
        Skin3dBundle {
            skinned_mesh: SkinnedMesh {
                inverse_bindposes: self.asset.into_handle(),
                joints: self.joints.into_joints(),
            },
        }
    }
}

#[derive(Bundle)]
pub struct Skin3dBundle {
    skinned_mesh: SkinnedMesh,
}

impl From<Handle<SkinnedMeshInverseBindposes>> for SkinAsset3d {
    fn from(inverse_bindposes: Handle<SkinnedMeshInverseBindposes>) -> Self {
        Self::new(inverse_bindposes)
    }
}
