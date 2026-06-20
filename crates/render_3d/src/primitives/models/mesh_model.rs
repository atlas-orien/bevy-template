use bevy::prelude::*;

use crate::primitives::meshes::MeshAsset3d;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct Model3dMarker;

#[derive(Debug, Clone)]
pub struct Model3d {
    mesh: MeshAsset3d,
    material: Handle<StandardMaterial>,
}

impl Model3d {
    pub fn new(mesh: impl Into<MeshAsset3d>, material: Handle<StandardMaterial>) -> Self {
        Self {
            mesh: mesh.into(),
            material,
        }
    }

    pub fn mesh(&self) -> &MeshAsset3d {
        &self.mesh
    }

    pub fn material(&self) -> &Handle<StandardMaterial> {
        &self.material
    }

    pub fn with_mesh(mut self, mesh: impl Into<MeshAsset3d>) -> Self {
        self.mesh = mesh.into();
        self
    }

    pub fn with_material(mut self, material: Handle<StandardMaterial>) -> Self {
        self.material = material;
        self
    }

    pub fn into_bundle(self, transform: Transform) -> Model3dBundle {
        Model3dBundle {
            marker: Model3dMarker,
            mesh: self.mesh.into_component(),
            material: MeshMaterial3d(self.material),
            transform,
            visibility: Visibility::default(),
        }
    }
}

#[derive(Bundle)]
pub struct Model3dBundle {
    marker: Model3dMarker,
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    transform: Transform,
    visibility: Visibility,
}
