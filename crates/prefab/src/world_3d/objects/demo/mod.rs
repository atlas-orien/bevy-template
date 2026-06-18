//! Demo 3D object prefabs.

pub mod capsule;
pub mod cube;
pub mod floor;
pub mod sphere;

pub use capsule::{DemoPreviewCapsule3dMarker, DemoPreviewCapsule3dPrefab};
pub use cube::{DemoPreviewCube3dMarker, DemoPreviewCube3dPrefab};
pub use floor::{DemoPreviewFloor3dMarker, DemoPreviewFloor3dPrefab};
pub use sphere::{DemoPreviewSphere3dMarker, DemoPreviewSphere3dPrefab};
