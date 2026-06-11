pub mod camera;
pub mod menu;

use bevy::prelude::*;

pub use camera::{
    UiLayer, UiRoot, full_screen_ui_node, ui_root_node_bundle, ui_root_target_bundle,
};
pub use menu::{demo_menu_button_node, demo_menu_root_node};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, _app: &mut App) {}
}
