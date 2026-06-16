use bevy::prelude::*;

use super::demo_menu_button::apply_demo_menu_focus_system;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_demo_menu_focus_system);
    }
}
