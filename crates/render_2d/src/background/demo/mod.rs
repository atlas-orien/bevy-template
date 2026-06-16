//! Demo 视差背景产品，包含背景 root、层级结构和视差系统。

mod entry;
mod layers;
mod systems;

pub use entry::DemoBackground2d;
pub(super) use systems::demo_parallax_background_system;
