use bevy::prelude::*;
use prefab::world_3d::{
    DemoPreviewCamera3dPrefab, DemoPreviewItems3dPrefab, DemoPreviewLights3dPrefab,
    DemoPreviewMaterials3d, DemoPreviewMeshes3d,
};
use render_3d::primitives::materials::StandardSurface3d;

pub struct DemoPreviewCamera3d;

impl DemoPreviewCamera3d {
    pub fn prefab() -> DemoPreviewCamera3dPrefab {
        DemoPreviewCamera3dPrefab
    }
}

pub struct DemoPreviewLights3d;

impl DemoPreviewLights3d {
    pub fn prefab() -> DemoPreviewLights3dPrefab {
        DemoPreviewLights3dPrefab
    }
}

pub struct DemoPreviewItems3d;

impl DemoPreviewItems3d {
    pub fn prefab(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> DemoPreviewItems3dPrefab {
        DemoPreviewItems3dPrefab::new(
            DemoPreviewMeshes3d {
                floor: meshes.add(Plane3d::default().mesh().size(8.0, 8.0)),
                cube: meshes.add(Cuboid::new(1.2, 1.2, 1.2)),
                sphere: meshes.add(Sphere::new(0.72)),
                capsule: meshes.add(Capsule3d::new(0.42, 1.4)),
            },
            DemoPreviewMaterials3d {
                floor: materials.add(
                    StandardSurface3d::new(Color::srgb(0.28, 0.32, 0.34))
                        .with_roughness(0.82)
                        .into_material(),
                ),
                cube: materials.add(
                    StandardSurface3d::new(Color::srgb(0.92, 0.42, 0.24))
                        .with_roughness(0.5)
                        .into_material(),
                ),
                sphere: materials.add(
                    StandardSurface3d::new(Color::srgb(0.25, 0.68, 0.95))
                        .with_roughness(0.35)
                        .with_metallic(0.15)
                        .into_material(),
                ),
                capsule: materials.add(
                    StandardSurface3d::new(Color::srgb(0.38, 0.86, 0.58))
                        .with_roughness(0.58)
                        .into_material(),
                ),
            },
        )
    }
}
