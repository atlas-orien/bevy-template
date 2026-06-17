//! 具体 3D 表现产品。

pub mod characters;
pub mod debug;
pub mod environment;
pub mod items;
pub mod overlays;
pub mod props;
pub mod scenes;

pub use characters::Characters3dPlugin;
pub use debug::Debug3dPlugin;
pub use environment::Environment3dPlugin;
pub use items::Items3dPlugin;
pub use overlays::Overlays3dPlugin;
pub use props::Props3dPlugin;
pub use scenes::Scenes3dPlugin;

use bevy::prelude::*;

pub struct Render3dProductsPlugin;

impl Plugin for Render3dProductsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            Scenes3dPlugin,
            Characters3dPlugin,
            Items3dPlugin,
            Props3dPlugin,
            Environment3dPlugin,
            Overlays3dPlugin,
            Debug3dPlugin,
        ));
    }
}
