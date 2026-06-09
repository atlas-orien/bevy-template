//! ECS 全局资源数据。
//!
//! 这里放 `#[derive(Resource)]` 的 Rust 数据类型。
//! 它们不是 `assets/` 文件，而是 Bevy World 里通常只有一份的全局数据。

pub mod session;
pub mod world;

use bevy::prelude::*;

use self::session::GameSession;
use self::world::WorldConfig;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldConfig>()
            .init_resource::<GameSession>();
    }
}
