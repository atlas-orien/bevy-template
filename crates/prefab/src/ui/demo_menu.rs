use bevy::prelude::*;
use interaction::InteractionAction;
use render_2d::ui::{
    DemoMenuButtonBundle, DemoMenuButtonIndex, DemoMenuButtonTextBundle, DemoMenuFocused,
    DemoMenuRootBundle, UiRootBundle,
};

use crate::Prefab;

pub const DEMO_START_ACTION: &str = "ui.demo.start";
pub const DEMO_OPTIONS_ACTION: &str = "ui.demo.options";
pub const DEMO_QUIT_ACTION: &str = "ui.demo.quit";
pub const DEMO_BACK_ACTION: &str = "ui.demo.back";

pub struct DemoMenuPrefab;

impl Prefab for DemoMenuPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands
            .spawn((
                UiRootBundle::default(),
                DemoMenuRootBundle::default(),
                children![
                    Self::button(0, "Start", DEMO_START_ACTION),
                    Self::button(1, "Options", DEMO_OPTIONS_ACTION),
                    Self::button(2, "Quit", DEMO_QUIT_ACTION),
                    Self::button(3, "Back", DEMO_BACK_ACTION),
                ],
            ))
            .id()
    }
}

impl DemoMenuPrefab {
    fn button(index: usize, label: &'static str, action: &'static str) -> impl Bundle {
        (
            DemoMenuButtonBundle::default(),
            DemoMenuButtonIndex(index),
            if index == 0 {
                DemoMenuFocused::focused()
            } else {
                DemoMenuFocused::unfocused()
            },
            InteractionAction::new(action),
            children![DemoMenuButtonTextBundle::new(label)],
        )
    }
}
