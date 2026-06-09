pub mod identity;
pub mod intent;
pub mod lifecycle;
pub mod prefab;
pub mod ui;
pub mod world_2d;
pub mod world_3d;

pub use error::Result;
pub use prefab::Prefab;

use bevy::prelude::*;
use ecs::EcsPlugin;
use physics::PhysicsPlugin;
use render_2d::Render2dPlugin;

pub struct PrefabPlugin;

impl Plugin for PrefabPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EcsPlugin, PhysicsPlugin, Render2dPlugin));
    }
}
