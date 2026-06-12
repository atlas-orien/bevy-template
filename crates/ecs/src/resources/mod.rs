//! ECS 全局资源数据。
//!
//! 这里放 `#[derive(Resource)]` 的 Rust 数据类型。
//! 它们不是 `assets/` 文件，而是 Bevy World 里通常只有一份的全局数据。

mod plugin;
pub mod session;
pub mod world;

pub use plugin::ResourcesPlugin;
