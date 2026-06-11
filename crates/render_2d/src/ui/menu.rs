use bevy::prelude::*;

use super::full_screen_ui_node;

pub fn demo_menu_root_node() -> Node {
    Node {
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        row_gap: px(12),
        justify_content: JustifyContent::Center,
        ..full_screen_ui_node()
    }
}

pub fn demo_menu_button_node(label: &'static str) -> impl Bundle {
    (
        Button,
        Node {
            width: px(220),
            height: px(56),
            border: UiRect::all(px(2)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BorderColor::all(Color::srgb(0.9, 0.9, 0.9)),
        BackgroundColor(Color::srgb(0.12, 0.14, 0.18)),
        children![(
            Text::new(label),
            TextFont {
                font_size: 22.0,
                ..default()
            },
            TextColor(Color::srgb(0.95, 0.96, 0.98)),
        )],
    )
}
