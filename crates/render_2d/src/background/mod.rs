pub mod demo_background;
mod plugin;

pub use demo_background::{
    DemoBackgroundLayer2d, DemoBackgroundLayer2dBundle, DemoParallaxBackgroundLayer2d,
    demo_parallax_background_system,
};
pub use plugin::BackgroundPlugin;
