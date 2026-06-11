use bevy::prelude::*;

use super::FullScreenUiNodeBundle;

#[derive(Bundle)]
pub struct DemoMenuRootBundle {
    pub node: FullScreenUiNodeBundle,
    pub layout: Node,
}

impl Default for DemoMenuRootBundle {
    fn default() -> Self {
        Self {
            node: FullScreenUiNodeBundle::default(),
            layout: Node {
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: px(12),
                justify_content: JustifyContent::Center,
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
                ..default()
            },
            border: BorderColor::all(Color::srgb(0.9, 0.9, 0.9)),
            background: BackgroundColor(Color::srgb(0.12, 0.14, 0.18)),
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
            color: TextColor(Color::srgb(0.95, 0.96, 0.98)),
        }
    }
}
