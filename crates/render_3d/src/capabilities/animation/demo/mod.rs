//! Demo 3D glTF animation product.

mod entry;
mod systems;

pub use entry::DemoFox3dAnimationSet;
pub use systems::{
    DemoFox3dAnimationState, DemoFox3dAnimationStateSet, DemoFox3dAnimationSystemPlugin,
};
