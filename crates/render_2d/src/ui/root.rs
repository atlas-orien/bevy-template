use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct UiRoot;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum UiLayer {
    Root,
}

#[derive(Bundle)]
pub struct UiRootBundle {
    pub marker: UiRoot,
    pub z_index: ZIndex,
}

impl Default for UiRootBundle {
    fn default() -> Self {
        Self {
            marker: UiRoot,
            z_index: ui_layer_z_index(UiLayer::Root),
        }
    }
}

#[derive(Bundle)]
pub struct FullScreenUiNodeBundle {
    pub node: Node,
}

impl Default for FullScreenUiNodeBundle {
    fn default() -> Self {
        Self {
            node: Node {
                width: percent(100),
                height: percent(100),
                ..default()
            },
        }
    }
}

#[derive(Bundle, Default)]
pub struct UiRootNodeBundle {
    pub root: UiRootBundle,
    pub node: FullScreenUiNodeBundle,
}

fn ui_layer_z_index(layer: UiLayer) -> ZIndex {
    match layer {
        UiLayer::Root => ZIndex(0),
    }
}
