//! 具体 2D 表现产品。

pub mod background;
pub mod characters;
pub mod debug;
pub mod environment;
pub mod items;
pub mod overlays;
pub mod props;
pub mod screens;
pub mod transitions;
pub mod ui;

pub use background::BackgroundPlugin;
pub use characters::CharacterRenderPlugin;
pub use debug::DebugRenderPlugin;
pub use environment::EnvironmentPlugin;
pub use items::ItemsPlugin;
pub use overlays::OverlaysPlugin;
pub use props::PropsPlugin;
pub use screens::ScreensPlugin;
pub use transitions::TransitionsPlugin;
pub use ui::UiPlugin;

use bevy::prelude::*;

pub struct Render2dProductsPlugin;

impl Plugin for Render2dProductsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            BackgroundPlugin,
            CharacterRenderPlugin,
            ItemsPlugin,
            PropsPlugin,
            EnvironmentPlugin,
            OverlaysPlugin,
            TransitionsPlugin,
            DebugRenderPlugin,
            ScreensPlugin,
            UiPlugin,
        ));
    }
}
