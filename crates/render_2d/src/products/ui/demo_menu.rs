//! Demo 菜单的 UI 节点 bundle、颜色样式与焦点视觉系统。

use bevy::prelude::*;

use super::root::UiRootBundle;

#[derive(Bundle, Default)]
pub struct DemoMenuVisual {
    root: UiRootBundle,
    menu: DemoMenuRootBundle,
}

const DEMO_MENU_GRID_COLUMNS: u16 = 2;
const DEMO_MENU_GRID_ROWS: u16 = 2;
const DEMO_MENU_GRID_GAP_PX: f32 = 14.0;

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
