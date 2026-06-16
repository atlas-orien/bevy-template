pub mod player;
mod plugin;

pub use player::{DemoPlayerAnimation2d, demo_player_idle_animation};
pub(super) use plugin::FrameAnimationDemoPlugin;
