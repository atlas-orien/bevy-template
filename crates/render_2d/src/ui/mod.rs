pub mod menu;
pub mod root;

use bevy::prelude::*;

pub use menu::{DemoMenuButtonBundle, DemoMenuButtonTextBundle, DemoMenuRootBundle};
pub use root::{FullScreenUiNodeBundle, UiLayer, UiRoot, UiRootBundle, UiRootNodeBundle};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, _app: &mut App) {}
}
