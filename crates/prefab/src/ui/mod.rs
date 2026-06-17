//! Screen UI prefab namespace.
//!
//! UI prefabs are shared by 2D and 3D games because they live in screen space.

pub mod demo_menu;

pub use demo_menu::{
    DEMO_BACK_ACTION, DEMO_MENU_ITEMS, DEMO_OPTIONS_ACTION, DEMO_QUIT_ACTION, DEMO_START_ACTION,
    DemoMenuAction, DemoMenuButtonIndex, DemoMenuFocused, DemoMenuItem, DemoMenuPrefab,
    DemoMenuRootMarker,
};
