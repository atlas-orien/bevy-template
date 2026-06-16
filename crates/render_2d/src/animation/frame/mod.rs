pub mod base;
pub mod content;
pub mod demo;
mod plugin;

pub use base::{
    FrameAnimation2d, FrameAnimationBasePlugin, FrameAnimationClip2d, FrameAnimationFacingFlip2d,
    FrameAnimationHandle2d, FrameAnimationLoader2d, FrameAnimationManifest2d,
    FrameAnimationManifestLoader2d, FrameAnimationMovementClips2d,
};
pub use plugin::FrameAnimation2dPlugin;
