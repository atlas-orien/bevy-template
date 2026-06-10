use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct ExampleMeshVisual2d;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct ExampleProceduralShape2d {
    pub color: Color,
    pub z: f32,
}

#[derive(Bundle)]
pub struct ExampleMeshVisual2dBundle {
    pub marker: ExampleMeshVisual2d,
    pub shape: ExampleProceduralShape2d,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
}

impl ExampleMeshVisual2dBundle {
    pub fn new(
        mesh: Handle<Mesh>,
        material: Handle<ColorMaterial>,
        color: Color,
        translation: Vec3,
    ) -> Self {
        Self {
            marker: ExampleMeshVisual2d,
            shape: ExampleProceduralShape2d {
                color,
                z: translation.z,
            },
            mesh: Mesh2d(mesh),
            material: MeshMaterial2d(material),
            transform: Transform::from_translation(translation),
        }
    }
}
