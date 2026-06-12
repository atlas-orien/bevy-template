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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DemoMenuAction {
    Start,
    Options,
    Quit,
    Back,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DemoMenuItem {
    pub label: &'static str,
    pub action: DemoMenuAction,
}

impl DemoMenuAction {
    pub const fn id(self) -> &'static str {
        match self {
            Self::Start => DEMO_START_ACTION,
            Self::Options => DEMO_OPTIONS_ACTION,
            Self::Quit => DEMO_QUIT_ACTION,
            Self::Back => DEMO_BACK_ACTION,
        }
    }

    pub fn from_id(id: &str) -> Option<Self> {
        DEMO_MENU_ITEMS
            .iter()
            .find_map(|item| (item.action.id() == id).then_some(item.action))
    }
}

pub const DEMO_MENU_ITEMS: &[DemoMenuItem] = &[
    DemoMenuItem {
        label: "Start",
        action: DemoMenuAction::Start,
    },
    DemoMenuItem {
        label: "Options",
        action: DemoMenuAction::Options,
    },
    DemoMenuItem {
        label: "Quit",
        action: DemoMenuAction::Quit,
    },
    DemoMenuItem {
        label: "Back",
        action: DemoMenuAction::Back,
    },
];

pub struct DemoMenuPrefab;

impl Prefab for DemoMenuPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands
            .spawn((UiRootBundle::default(), DemoMenuRootBundle::default()))
            .with_children(|parent| {
                for (index, item) in DEMO_MENU_ITEMS.iter().enumerate() {
                    parent.spawn(Self::button(index, *item));
                }
            })
            .id()
    }
}

impl DemoMenuPrefab {
    fn button(index: usize, item: DemoMenuItem) -> impl Bundle {
        (
            DemoMenuButtonBundle::default(),
            DemoMenuButtonIndex(index),
            if index == 0 {
                DemoMenuFocused::focused()
            } else {
                DemoMenuFocused::unfocused()
            },
            InteractionAction::new(item.action.id()),
            children![DemoMenuButtonTextBundle::new(item.label)],
        )
    }
}
