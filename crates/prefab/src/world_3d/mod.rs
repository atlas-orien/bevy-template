//! 3D world prefab namespace.

pub mod cameras;
pub mod characters;
pub mod lights;
pub mod objects;

pub use cameras::{
    DemoPreviewCamera3dMarker, DemoPreviewCamera3dPrefab, DemoPreviewOrbitCamera3dPrefab,
};
pub use characters::{DemoFox3dPrefab, DemoFox3dPrefabMarker};
pub use lights::{DemoPreviewLights3dMarker, DemoPreviewLights3dPrefab};
pub use objects::{
    DemoPreviewCapsule3dMarker, DemoPreviewCapsule3dPrefab, DemoPreviewCube3dMarker,
    DemoPreviewCube3dPrefab, DemoPreviewFloor3dMarker, DemoPreviewFloor3dPrefab,
    DemoPreviewSphere3dMarker, DemoPreviewSphere3dPrefab,
};
