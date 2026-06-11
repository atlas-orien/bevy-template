pub mod camera;
pub mod menu;

use bevy::prelude::*;

pub use camera::{FullScreenUiNodeBundle, UiLayer, UiRoot, UiRootBundle, UiRootNodeBundle};
pub use menu::{DemoMenuButtonBundle, DemoMenuButtonTextBundle, DemoMenuRootBundle};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, _app: &mut App) {}
}
