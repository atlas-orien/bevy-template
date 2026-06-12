mod api;
mod plugin;

pub use api::set_movement_intent;
pub use plugin::MovementIntentPlugin;
pub use prefab::intent::{MovementIntentQuery, MovementTarget};
