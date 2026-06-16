pub mod demo_frame_animation;
pub mod demo_frame_manifest;
mod plugin;

pub use demo_frame_animation::{
    DemoFrameAnimation2d, DemoPlayerAnimation2d, demo_frame_animation_system,
    demo_player_animation_state_system,
};
pub use demo_frame_manifest::DemoFrameManifestHandle2d;
pub use demo_frame_manifest::{DemoFrameClip2d, DemoFrameManifest2d, DemoFrameManifestLoader2d};
pub use plugin::FrameAnimation2dPlugin;
