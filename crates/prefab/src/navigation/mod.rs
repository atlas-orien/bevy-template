mod api;

pub use ::navigation::{
    NavigationAgent2d, NavigationAgent3d, NavigationPath2d, NavigationPath3d, NavigationTarget2d,
    NavigationTarget3d,
};
pub use api::{
    Navigation2dBundle, Navigation3dBundle, NavigationTarget2dQuery, NavigationTarget3dQuery,
    set_navigation_target_2d, set_navigation_target_3d,
    sync_demo_navigation_targets_from_intent_system,
};
