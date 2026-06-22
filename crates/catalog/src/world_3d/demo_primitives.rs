use bevy::prelude::*;
use helper::assets::TextureAsset;
use prefab::world_3d::{
    DemoPreviewCamera3dPrefab, DemoPreviewCapsule3dPrefab, DemoPreviewCube3dPrefab,
    DemoPreviewFloor3dPrefab, DemoPreviewLights3dPrefab, DemoPreviewOrbitCamera3dPrefab,
    DemoPreviewSphere3dPrefab,
};
use render_3d::primitives::materials::MaterialSurface3d;

const DEMO_METAL_BASE_COLOR: TextureAsset =
    TextureAsset::srgb("3d/materials/demo-metal/base-color.png");
const DEMO_METAL_NORMAL: TextureAsset = TextureAsset::linear("3d/materials/demo-metal/normal.png");
const DEMO_METAL_OCCLUSION_ROUGH_METAL: TextureAsset =
    TextureAsset::linear("3d/materials/demo-metal/occlusion-rough-metal.png");

pub struct DemoPreviewCamera3d;

impl DemoPreviewCamera3d {
    pub fn prefab() -> DemoPreviewCamera3dPrefab {
        DemoPreviewCamera3dPrefab
    }

    pub fn orbit_prefab() -> DemoPreviewOrbitCamera3dPrefab {
        DemoPreviewOrbitCamera3dPrefab
    }
}

pub struct DemoPreviewLights3d;

impl DemoPreviewLights3d {
    pub fn prefab() -> DemoPreviewLights3dPrefab {
        DemoPreviewLights3dPrefab
    }
}

pub struct DemoPreviewFloor3d;

impl DemoPreviewFloor3d {
    pub fn prefab(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> DemoPreviewFloor3dPrefab {
        DemoPreviewFloor3dPrefab::new(
            meshes.add(Plane3d::default().mesh().size(8.0, 8.0)),
            materials.add(
                MaterialSurface3d::flat_color(Color::srgb(0.28, 0.32, 0.34)).with_roughness(0.82),
            ),
        )
    }
}

pub struct DemoPreviewCube3d;

impl DemoPreviewCube3d {
    pub fn prefab(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> DemoPreviewCube3dPrefab {
        DemoPreviewCube3dPrefab::new(
            meshes.add(Cuboid::new(1.2, 1.2, 1.2)),
            materials.add(
                MaterialSurface3d::flat_color(Color::srgb(0.92, 0.42, 0.24)).with_roughness(0.5),
            ),
        )
    }
}

pub struct DemoPreviewSphere3d;

impl DemoPreviewSphere3d {
    pub fn prefab(
        asset_server: &AssetServer,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> DemoPreviewSphere3dPrefab {
        DemoPreviewSphere3dPrefab::new(
            meshes.add(Sphere::new(0.72).mesh().uv(48, 24)),
            materials.add(
                MaterialSurface3d::empty()
                    .with_textured_pbr(
                        DEMO_METAL_BASE_COLOR.load(asset_server),
                        DEMO_METAL_NORMAL.load(asset_server),
                        DEMO_METAL_OCCLUSION_ROUGH_METAL.load(asset_server),
                    )
                    .with_roughness(1.0)
                    .with_metallic(1.0),
            ),
        )
    }
}

pub struct DemoPreviewCapsule3d;

impl DemoPreviewCapsule3d {
    pub fn prefab(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> DemoPreviewCapsule3dPrefab {
        DemoPreviewCapsule3dPrefab::new(
            meshes.add(Capsule3d::new(0.42, 1.4)),
            materials.add(
                MaterialSurface3d::flat_color(Color::srgb(0.38, 0.86, 0.58)).with_roughness(0.58),
            ),
        )
    }
}
