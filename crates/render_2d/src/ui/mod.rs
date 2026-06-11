pub mod demo_menu;
pub mod root;

use bevy::prelude::*;

pub use demo_menu::{DemoMenuButtonBundle, DemoMenuButtonTextBundle, DemoMenuRootBundle};
pub use root::{FullScreenUiNodeBundle, UiLayer, UiRoot, UiRootBundle, UiRootNodeBundle};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, _app: &mut App) {}
}
