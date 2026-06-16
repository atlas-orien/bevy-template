//! 通用分层背景能力：颜色层、图片层和 camera parallax。

mod entry;
mod layers;
mod systems;

pub use entry::{BackgroundLayer2d, LayeredBackground2d, LayeredBackgroundRoot2d};
pub(super) use systems::layered_background_parallax_system;
