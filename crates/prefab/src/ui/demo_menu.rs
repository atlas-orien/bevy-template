//! Demo 菜单 prefab：菜单项表是按钮 label、action 与数量的单一来源。

use bevy::prelude::*;
use interaction::InteractionAction;
use render_2d::products::ui::{
    DemoMenuButtonTextVisualBundle, DemoMenuButtonVisual, DemoMenuVisual,
};

use crate::Prefab;

pub const DEMO_START_ACTION: &str = "ui.demo.start";
pub const DEMO_OPTIONS_ACTION: &str = "ui.demo.options";
pub const DEMO_NETWORK_LOGIN_ACTION: &str = "ui.demo.network_login";
pub const DEMO_QUIT_ACTION: &str = "ui.demo.quit";
pub const DEMO_BACK_ACTION: &str = "ui.demo.back";

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DemoMenuAction {
    Start,
    Options,
    NetworkLogin,
    Quit,
    Back,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DemoMenuItem {
    pub label: &'static str,
    pub action: DemoMenuAction,
}

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq)]
pub struct DemoMenuButtonIndex(pub usize);

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq)]
pub struct DemoMenuFocused {
    pub focused: bool,
}

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq)]
pub struct DemoMenuRootMarker;

#[derive(Bundle)]
struct DemoMenuBundle {
    root: DemoMenuRootMarker,
    visual: DemoMenuVisual,
}

impl Default for DemoMenuBundle {
    fn default() -> Self {
        Self {
            root: DemoMenuRootMarker,
            visual: DemoMenuVisual::default(),
        }
    }
}

impl DemoMenuFocused {
    pub const fn focused() -> Self {
        Self { focused: true }
    }

    pub const fn unfocused() -> Self {
        Self { focused: false }
    }
}

impl DemoMenuAction {
    pub const fn id(self) -> &'static str {
        match self {
            Self::Start => DEMO_START_ACTION,
            Self::Options => DEMO_OPTIONS_ACTION,
            Self::NetworkLogin => DEMO_NETWORK_LOGIN_ACTION,
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
        label: "Test Network",
        action: DemoMenuAction::NetworkLogin,
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
            .spawn(DemoMenuBundle::default())
            .with_children(|parent| {
                for (index, item) in DEMO_MENU_ITEMS.iter().enumerate() {
                    parent
                        .spawn(DemoMenuButtonBundle::new(index, *item))
                        .with_children(|button| {
                            button.spawn(DemoMenuButtonTextVisualBundle::new(item.label));
                        });
                }
            })
            .id()
    }
}

#[derive(Bundle)]
struct DemoMenuButtonBundle {
    index: DemoMenuButtonIndex,
    focused: DemoMenuFocused,
    action: InteractionAction,
    visual: render_2d::products::ui::DemoMenuButtonVisualBundle,
}

impl DemoMenuButtonBundle {
    fn new(index: usize, item: DemoMenuItem) -> Self {
        let focused = index == 0;

        Self {
            index: DemoMenuButtonIndex(index),
            focused: if focused {
                DemoMenuFocused::focused()
            } else {
                DemoMenuFocused::unfocused()
            },
            action: InteractionAction::new(item.action.id()),
            visual: DemoMenuButtonVisual::new(focused).into_bundle(),
        }
    }
}
