use bevy::prelude::*;
use interaction::InteractionAction;
use render_2d::ui::{
    DemoMenuButtonBundle, DemoMenuButtonTextBundle, DemoMenuRootBundle, UiRootBundle,
};

use crate::Prefab;

pub const DEMO_START_ACTION: &str = "ui.demo.start";
pub const DEMO_OPTIONS_ACTION: &str = "ui.demo.options";
pub const DEMO_QUIT_ACTION: &str = "ui.demo.quit";

pub struct DemoMenuPrefab;

impl Prefab for DemoMenuPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands
            .spawn((
                UiRootBundle::default(),
                DemoMenuRootBundle::default(),
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
    (
        DemoMenuButtonBundle::default(),
        InteractionAction::new(action),
        children![DemoMenuButtonTextBundle::new(label)],
    )
}
