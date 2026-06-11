use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct UiRoot;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum UiLayer {
    Root,
}

pub fn ui_root_node_bundle(camera: Entity) -> impl Bundle {
    (ui_root_target_bundle(camera), full_screen_ui_node())
}

pub fn ui_root_target_bundle(camera: Entity) -> impl Bundle {
    (
        UiRoot,
        UiTargetCamera(camera),
        ui_layer_z_index(UiLayer::Root),
    )
}

pub fn full_screen_ui_node() -> Node {
    Node {
        width: percent(100),
        height: percent(100),
        ..default()
    }
}

fn ui_layer_z_index(layer: UiLayer) -> ZIndex {
    match layer {
        UiLayer::Root => ZIndex(0),
    }
}
