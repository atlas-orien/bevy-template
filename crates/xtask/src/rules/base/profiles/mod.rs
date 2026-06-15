pub mod app;
pub mod dev_preview;
pub mod ecs;
pub mod error;
pub mod foundation;
pub mod gameplay;
pub mod network;
pub mod physics;
pub mod prefab;
pub mod render;
pub mod runtime;

pub use app::{AppRules, check_app};
pub use dev_preview::{DevPreviewRules, check_dev_preview};
pub use ecs::{EcsRules, check_ecs};
pub use error::{ErrorRules, check_error};
pub use foundation::{SimpleCrateRules, check_simple_crate};
pub use gameplay::{
    GameplayRules, IntentRules, NavigationRules, check_gameplay, check_intent, check_navigation,
};
pub use network::{NetworkRules, check_network};
pub use physics::{PhysicsRules, check_physics};
pub use prefab::{PrefabRules, check_prefab};
pub use render::{Render2dRules, Render3dRules, check_render_2d, check_render_3d};
pub use runtime::{
    ExternalRuntimeRules, InteractionRules, PeripheralsRules, check_external_runtime,
    check_interaction, check_peripherals,
};
