//! Demo 3D primitives prefabs。

use bevy::ecs::spawn::{Spawn, SpawnRelatedBundle};
use bevy::prelude::*;
use render_3d::primitives::camera::FixedCamera3dBundle;
use render_3d::primitives::lights::{
    DirectionalLight3dBundle, PointLight3dBundle, SpotLight3dBundle,
};
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
#[bundle(ignore_from_components)]
struct DemoPreviewLights3dBundle {
    marker: DemoPreviewLights3dMarker,
    transform: Transform,
    visibility: Visibility,
    children: DemoPreviewLights3dChildrenBundle,
}

impl Default for DemoPreviewLights3dBundle {
    fn default() -> Self {
        Self {
            marker: DemoPreviewLights3dMarker,
            transform: Transform::default(),
            visibility: Visibility::default(),
            children: Children::spawn((
                Spawn(DirectionalLight3dBundle::default()),
                Spawn(PointLight3dBundle::new(
                    PointLight {
                        intensity: 420_000.0,
                        range: 18.0,
                        color: Color::srgb(0.86, 0.94, 1.0),
                        shadows_enabled: true,
                        ..default()
                    },
                    Vec3::new(-3.0, 4.5, 2.0),
                )),
                Spawn(SpotLight3dBundle::new(
                    SpotLight {
                        intensity: 950_000.0,
                        range: 16.0,
                        color: Color::srgb(1.0, 0.72, 0.48),
                        shadows_enabled: true,
                        inner_angle: 0.35,
                        outer_angle: 0.75,
                        ..default()
                    },
                    Vec3::new(3.8, 5.0, 4.0),
                    Vec3::ZERO,
                )),
            )),
        }
    }
}

type DemoPreviewLights3dChildrenBundle = SpawnRelatedBundle<
    ChildOf,
    (
        Spawn<DirectionalLight3dBundle>,
        Spawn<PointLight3dBundle>,
        Spawn<SpotLight3dBundle>,
    ),
>;

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
