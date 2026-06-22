//! 3D world catalog namespace.

mod demo_fox;
mod demo_primitives;

pub use demo_fox::DemoFox3d;
pub use demo_primitives::{
    DemoPreviewCamera3d, DemoPreviewCapsule3d, DemoPreviewCube3d, DemoPreviewFloor3d,
    DemoPreviewLights3d, DemoPreviewSphere3d,
};
