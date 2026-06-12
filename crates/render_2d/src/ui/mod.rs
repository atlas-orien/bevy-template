pub mod demo_menu;
pub mod root;

use bevy::prelude::*;

use self::demo_menu::apply_demo_menu_focus_system;

pub use demo_menu::{
    DemoMenuButtonBundle, DemoMenuButtonIndex, DemoMenuButtonTextBundle, DemoMenuFocused,
    DemoMenuRoot, DemoMenuRootBundle,
};
pub use root::{FullScreenUiNodeBundle, UiLayer, UiRoot, UiRootBundle, UiRootNodeBundle};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_demo_menu_focus_system);
    }
}
