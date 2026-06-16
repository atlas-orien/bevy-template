//! Demo 菜单按钮视觉表现。

use bevy::prelude::*;

const DEMO_MENU_NORMAL_BACKGROUND: Color = Color::srgb(0.12, 0.14, 0.18);
const DEMO_MENU_FOCUSED_BACKGROUND: Color = Color::srgb(0.18, 0.30, 0.48);
const DEMO_MENU_NORMAL_BORDER: Color = Color::srgb(0.42, 0.46, 0.52);
const DEMO_MENU_FOCUSED_BORDER: Color = Color::srgb(0.98, 0.82, 0.32);
const DEMO_MENU_TEXT: Color = Color::srgb(0.95, 0.96, 0.98);
const DEMO_MENU_BUTTON_WIDTH_PX: f32 = 220.0;
const DEMO_MENU_BUTTON_HEIGHT_PX: f32 = 56.0;
const DEMO_MENU_BUTTON_BORDER_PX: f32 = 2.0;
const DEMO_MENU_BUTTON_FONT_SIZE: f32 = 22.0;

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq)]
pub struct DemoMenuVisualFocused {
    pub focused: bool,
}

impl DemoMenuVisualFocused {
    pub const fn focused() -> Self {
        Self { focused: true }
    }

    pub const fn unfocused() -> Self {
        Self { focused: false }
    }
}

pub struct DemoMenuButtonVisual {
    label: &'static str,
    focused: bool,
}

impl DemoMenuButtonVisual {
    pub const fn new(label: &'static str, focused: bool) -> Self {
        Self { label, focused }
    }

    pub fn into_bundle(self) -> DemoMenuButtonVisualBundle {
        DemoMenuButtonVisualBundle::new(self.label, self.focused)
    }
}

#[derive(Bundle)]
#[bundle(ignore_from_components)]
pub struct DemoMenuButtonVisualBundle(
    DemoMenuButtonBundle,
    DemoMenuVisualFocused,
    bevy::ecs::spawn::SpawnRelatedBundle<
        bevy::ecs::hierarchy::ChildOf,
        bevy::ecs::spawn::Spawn<DemoMenuButtonTextBundle>,
    >,
);

impl DemoMenuButtonVisualBundle {
    pub fn new(label: &'static str, focused: bool) -> Self {
        Self(
            DemoMenuButtonBundle::default(),
            if focused {
                DemoMenuVisualFocused::focused()
            } else {
                DemoMenuVisualFocused::unfocused()
            },
            children![DemoMenuButtonTextBundle::new(label)],
        )
    }
}

#[derive(Bundle)]
struct DemoMenuButtonBundle {
    button: Button,
    node: Node,
    border: BorderColor,
    background: BackgroundColor,
}

impl Default for DemoMenuButtonBundle {
    fn default() -> Self {
        Self {
            button: Button,
            node: Node {
                width: px(DEMO_MENU_BUTTON_WIDTH_PX),
                height: px(DEMO_MENU_BUTTON_HEIGHT_PX),
                border: UiRect::all(px(DEMO_MENU_BUTTON_BORDER_PX)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                ..default()
            },
            border: BorderColor::all(DEMO_MENU_NORMAL_BORDER),
            background: BackgroundColor(DEMO_MENU_NORMAL_BACKGROUND),
        }
    }
}

#[derive(Bundle)]
struct DemoMenuButtonTextBundle {
    text: Text,
    font: TextFont,
    color: TextColor,
}

impl DemoMenuButtonTextBundle {
    fn new(label: impl Into<String>) -> Self {
        Self {
            text: Text::new(label),
            font: TextFont {
                font_size: DEMO_MENU_BUTTON_FONT_SIZE,
                ..default()
            },
            color: TextColor(DEMO_MENU_TEXT),
        }
    }
}

pub type DemoMenuFocusQuery<'world, 'state> = Query<
    'world,
    'state,
    (
        &'static DemoMenuVisualFocused,
        &'static mut BackgroundColor,
        &'static mut BorderColor,
    ),
    Changed<DemoMenuVisualFocused>,
>;

pub fn apply_demo_menu_focus_system(mut buttons: DemoMenuFocusQuery) {
    for (focus, mut background, mut border) in &mut buttons {
        if focus.focused {
            background.0 = DEMO_MENU_FOCUSED_BACKGROUND;
            *border = BorderColor::all(DEMO_MENU_FOCUSED_BORDER);
        } else {
            background.0 = DEMO_MENU_NORMAL_BACKGROUND;
            *border = BorderColor::all(DEMO_MENU_NORMAL_BORDER);
        }
    }
}
