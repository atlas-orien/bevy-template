//! 通用 2D layer 组合能力，供 background、environment、screens 等复合表现使用。

mod entry;
mod plugin;
mod stack;
mod systems;

pub use entry::{LayerStack2d, RenderLayer2d};
pub use plugin::Layers2dPlugin;
