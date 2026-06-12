pub mod demo_frame_animation;
mod plugin;

pub use demo_frame_animation::{
    DemoFrameAnimation2d, DemoPlayerAnimation2d, demo_frame_animation_system,
    demo_player_animation_state_system,
};
pub use plugin::FrameAnimation2dPlugin;
