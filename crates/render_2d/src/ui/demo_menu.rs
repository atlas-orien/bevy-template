use bevy::prelude::*;

const DEMO_MENU_NORMAL_BACKGROUND: Color = Color::srgb(0.12, 0.14, 0.18);
const DEMO_MENU_FOCUSED_BACKGROUND: Color = Color::srgb(0.18, 0.30, 0.48);
const DEMO_MENU_NORMAL_BORDER: Color = Color::srgb(0.42, 0.46, 0.52);
const DEMO_MENU_FOCUSED_BORDER: Color = Color::srgb(0.98, 0.82, 0.32);
const DEMO_MENU_TEXT: Color = Color::srgb(0.95, 0.96, 0.98);

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq)]
pub struct DemoMenuButtonIndex(pub usize);

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq)]
pub struct DemoMenuFocused {
    pub focused: bool,
}

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq)]
pub struct DemoMenuRoot;

impl DemoMenuFocused {
    pub const fn focused() -> Self {
        Self { focused: true }
    }

    pub const fn unfocused() -> Self {
        Self { focused: false }
    }
}

#[derive(Bundle)]
pub struct DemoMenuRootBundle {
    pub marker: DemoMenuRoot,
    pub node: Node,
}

impl Default for DemoMenuRootBundle {
    fn default() -> Self {
        Self {
            marker: DemoMenuRoot,
            node: Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                display: Display::Grid,
                grid_template_columns: RepeatedGridTrack::auto(2),
                grid_template_rows: RepeatedGridTrack::auto(2),
                row_gap: px(14),
                column_gap: px(14),
                justify_content: JustifyContent::Center,
                justify_items: JustifyItems::Center,
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct DemoMenuButtonBundle {
    pub button: Button,
    pub node: Node,
    pub border: BorderColor,
    pub background: BackgroundColor,
}

impl Default for DemoMenuButtonBundle {
    fn default() -> Self {
        Self {
            button: Button,
            node: Node {
                width: px(220),
                height: px(56),
                border: UiRect::all(px(2)),
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
pub struct DemoMenuButtonTextBundle {
    pub text: Text,
    pub font: TextFont,
    pub color: TextColor,
}

impl DemoMenuButtonTextBundle {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            text: Text::new(label),
            font: TextFont {
                font_size: 22.0,
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
        &'static DemoMenuFocused,
        &'static mut BackgroundColor,
        &'static mut BorderColor,
    ),
    Changed<DemoMenuFocused>,
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
