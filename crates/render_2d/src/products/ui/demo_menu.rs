//! Demo 菜单的 UI 节点 bundle、颜色样式与焦点视觉系统。

use bevy::prelude::*;

use super::root::UiRootBundle;

const DEMO_MENU_GRID_COLUMNS: u16 = 2;
const DEMO_MENU_GRID_ROWS: u16 = 2;
const DEMO_MENU_GRID_GAP_PX: f32 = 14.0;
const DEMO_MENU_NORMAL_BACKGROUND: Color = Color::srgb(0.12, 0.14, 0.18);
const DEMO_MENU_FOCUSED_BACKGROUND: Color = Color::srgb(0.18, 0.30, 0.48);
const DEMO_MENU_NORMAL_BORDER: Color = Color::srgb(0.42, 0.46, 0.52);
const DEMO_MENU_FOCUSED_BORDER: Color = Color::srgb(0.98, 0.82, 0.32);
const DEMO_MENU_TEXT: Color = Color::srgb(0.95, 0.96, 0.98);
const DEMO_MENU_BUTTON_WIDTH_PX: f32 = 220.0;
const DEMO_MENU_BUTTON_HEIGHT_PX: f32 = 56.0;
const DEMO_MENU_BUTTON_BORDER_PX: f32 = 2.0;
const DEMO_MENU_BUTTON_FONT_SIZE: f32 = 22.0;

#[derive(Bundle, Default)]
pub struct DemoMenuVisual {
    root: UiRootBundle,
    menu: DemoMenuRootBundle,
}

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq)]
struct DemoMenuRootVisualMarker;

#[derive(Bundle)]
pub(crate) struct DemoMenuRootBundle {
    marker: DemoMenuRootVisualMarker,
    node: Node,
}

impl Default for DemoMenuRootBundle {
    fn default() -> Self {
        Self {
            marker: DemoMenuRootVisualMarker,
            node: Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                display: Display::Grid,
                grid_template_columns: RepeatedGridTrack::auto(DEMO_MENU_GRID_COLUMNS),
                grid_template_rows: RepeatedGridTrack::auto(DEMO_MENU_GRID_ROWS),
                row_gap: px(DEMO_MENU_GRID_GAP_PX),
                column_gap: px(DEMO_MENU_GRID_GAP_PX),
                justify_content: JustifyContent::Center,
                justify_items: JustifyItems::Center,
                ..default()
            },
        }
    }
}

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
    focused: bool,
}

impl DemoMenuButtonVisual {
    pub const fn new(focused: bool) -> Self {
        Self { focused }
    }

    pub fn into_bundle(self) -> DemoMenuButtonVisualBundle {
        DemoMenuButtonVisualBundle::new(self.focused)
    }
}

#[derive(Bundle, Default)]
pub struct DemoMenuButtonTextVisualBundle {
    text: Text,
    font: TextFont,
    color: TextColor,
}

impl DemoMenuButtonTextVisualBundle {
    pub fn new(label: &'static str) -> Self {
        Self {
            text: Text::new(label),
            font: TextFont::from_font_size(DEMO_MENU_BUTTON_FONT_SIZE),
            color: TextColor(DEMO_MENU_TEXT),
        }
    }
}

#[derive(Bundle)]
pub struct DemoMenuButtonVisualBundle {
    button: Button,
    node: Node,
    border: BorderColor,
    background: BackgroundColor,
    focused: DemoMenuVisualFocused,
}

impl DemoMenuButtonVisualBundle {
    pub fn new(focused: bool) -> Self {
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
            focused: if focused {
                DemoMenuVisualFocused::focused()
            } else {
                DemoMenuVisualFocused::unfocused()
            },
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
