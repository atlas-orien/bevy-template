use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub(crate) struct UiRoot;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) enum UiLayer {
    Root,
}

#[derive(Bundle)]
pub(crate) struct UiRootBundle {
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

fn ui_layer_z_index(layer: UiLayer) -> ZIndex {
    match layer {
        UiLayer::Root => ZIndex(0),
    }
}
