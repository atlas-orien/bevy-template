use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct MeshAsset3d {
    mesh: Handle<Mesh>,
}

impl MeshAsset3d {
    pub fn new(mesh: Handle<Mesh>) -> Self {
        Self { mesh }
    }

    pub fn handle(&self) -> &Handle<Mesh> {
        &self.mesh
    }

    pub fn clone_handle(&self) -> Handle<Mesh> {
        self.mesh.clone()
    }

    pub fn into_handle(self) -> Handle<Mesh> {
        self.mesh
    }

    pub fn into_component(self) -> Mesh3d {
        Mesh3d(self.mesh)
    }
}

impl From<Handle<Mesh>> for MeshAsset3d {
    fn from(mesh: Handle<Mesh>) -> Self {
        Self::new(mesh)
    }
}

impl From<MeshAsset3d> for Handle<Mesh> {
    fn from(mesh: MeshAsset3d) -> Self {
        mesh.into_handle()
    }
}

impl From<MeshAsset3d> for Mesh3d {
    fn from(mesh: MeshAsset3d) -> Self {
        mesh.into_component()
    }
}
