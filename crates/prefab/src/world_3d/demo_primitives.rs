//! Demo 3D primitives prefabs。

use bevy::prelude::*;
use render_3d::primitives::camera::FixedCamera3dBundle;
use render_3d::primitives::lights::SunLight3dBundle;
use render_3d::primitives::meshes::StaticMesh3d;

use crate::Prefab;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoPreviewCamera3dMarker;

#[derive(Default)]
pub struct DemoPreviewCamera3dPrefab;

#[derive(Bundle, Default)]
struct DemoPreviewCamera3dBundle {
    marker: DemoPreviewCamera3dMarker,
    camera: FixedCamera3dBundle,
}

impl Prefab for DemoPreviewCamera3dPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands.spawn(DemoPreviewCamera3dBundle::default()).id()
    }
}

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoPreviewLights3dMarker;

#[derive(Default)]
pub struct DemoPreviewLights3dPrefab;

#[derive(Bundle)]
struct DemoPreviewLights3dBundle {
    marker: DemoPreviewLights3dMarker,
    sunlight: SunLight3dBundle,
}

impl Default for DemoPreviewLights3dBundle {
    fn default() -> Self {
        Self {
            marker: DemoPreviewLights3dMarker,
            sunlight: SunLight3dBundle::default(),
        }
    }
}

impl Prefab for DemoPreviewLights3dPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands.spawn(DemoPreviewLights3dBundle::default()).id()
    }
}

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoPreviewItems3dMarker;

pub struct DemoPreviewItems3dPrefab {
    meshes: DemoPreviewMeshes3d,
    materials: DemoPreviewMaterials3d,
}

impl DemoPreviewItems3dPrefab {
    pub fn new(meshes: DemoPreviewMeshes3d, materials: DemoPreviewMaterials3d) -> Self {
        Self { meshes, materials }
    }
}

impl Prefab for DemoPreviewItems3dPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        let root = commands.spawn(DemoPreviewItems3dRootBundle::default()).id();
        for item in demo_item_bundles(self.meshes, self.materials) {
            commands.spawn(item);
        }
        root
    }
}

#[derive(Clone)]
pub struct DemoPreviewMeshes3d {
    pub floor: Handle<Mesh>,
    pub cube: Handle<Mesh>,
    pub sphere: Handle<Mesh>,
    pub capsule: Handle<Mesh>,
}

#[derive(Clone)]
pub struct DemoPreviewMaterials3d {
    pub floor: Handle<StandardMaterial>,
    pub cube: Handle<StandardMaterial>,
    pub sphere: Handle<StandardMaterial>,
    pub capsule: Handle<StandardMaterial>,
}

#[derive(Bundle)]
struct DemoPreviewItems3dRootBundle {
    marker: DemoPreviewItems3dMarker,
    transform: Transform,
    visibility: Visibility,
}

impl Default for DemoPreviewItems3dRootBundle {
    fn default() -> Self {
        Self {
            marker: DemoPreviewItems3dMarker,
            transform: Transform::default(),
            visibility: Visibility::default(),
        }
    }
}

const DEMO_PREVIEW_ITEM_COUNT: usize = 4;

fn demo_item_bundles(
    meshes: DemoPreviewMeshes3d,
    materials: DemoPreviewMaterials3d,
) -> [impl Bundle; DEMO_PREVIEW_ITEM_COUNT] {
    [
        StaticMesh3d::at(meshes.floor, materials.floor, Vec3::ZERO).into_bundle(),
        StaticMesh3d::new(
            meshes.cube,
            materials.cube,
            Transform::from_xyz(-1.8, 0.6, 0.0).with_rotation(Quat::from_rotation_y(0.45)),
        )
        .into_bundle(),
        StaticMesh3d::at(meshes.sphere, materials.sphere, Vec3::new(0.0, 0.72, 0.0)).into_bundle(),
        StaticMesh3d::new(
            meshes.capsule,
            materials.capsule,
            Transform::from_xyz(1.85, 0.98, 0.0).with_rotation(Quat::from_rotation_z(0.16)),
        )
        .into_bundle(),
    ]
}
