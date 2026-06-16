pub mod base;
pub mod demo_player;
mod plugin;

pub use base::{
    FrameAnimation2d, FrameAnimationBasePlugin, FrameAnimationClip2d, FrameAnimationHandle2d,
    FrameAnimationLoader2d, FrameAnimationManifest2d, FrameAnimationManifestLoader2d,
};
pub use demo_player::{DemoPlayerAnimation2d, demo_player_idle_animation};
pub use plugin::FrameAnimation2dPlugin;
