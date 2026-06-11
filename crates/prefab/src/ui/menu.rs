use bevy::prelude::*;
use interaction::InteractionAction;
use render_2d::ui::{demo_menu_button_node, demo_menu_root_node, ui_root_target_bundle};

use crate::Prefab;

pub const DEMO_START_ACTION: &str = "ui.demo.start";
pub const DEMO_OPTIONS_ACTION: &str = "ui.demo.options";
pub const DEMO_QUIT_ACTION: &str = "ui.demo.quit";

pub struct DemoMenuPrefab {
    pub ui_camera: Entity,
}

impl Prefab for DemoMenuPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands
            .spawn((
                ui_root_target_bundle(self.ui_camera),
                demo_menu_root_node(),
                children![
                    demo_menu_button("Start", DEMO_START_ACTION),
                    demo_menu_button("Options", DEMO_OPTIONS_ACTION),
                    demo_menu_button("Quit", DEMO_QUIT_ACTION),
                ],
            ))
            .id()
    }
}

fn demo_menu_button(label: &'static str, action: &'static str) -> impl Bundle {
    (demo_menu_button_node(label), InteractionAction::new(action))
}
