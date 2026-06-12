pub mod control;
pub mod demo_events;
pub mod identity;
pub mod intent;
pub mod lifecycle;
pub mod movement;
pub mod navigation;
pub mod prefab;
pub mod ui;
pub mod world_2d;
pub mod world_3d;

pub use error::Result;
pub use prefab::Prefab;

use ::navigation::NavigationPlugin;
use audio::AudioFoundationPlugin;
use bevy::prelude::*;
use ecs::EcsPlugin;
use physics::PhysicsPlugin;
use render_2d::Render2dPlugin;

pub struct PrefabPlugin;

impl Plugin for PrefabPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EcsPlugin,
            PhysicsPlugin,
            NavigationPlugin,
            AudioFoundationPlugin,
            Render2dPlugin,
        ))
        .add_systems(
            Update,
            (
                lifecycle::play_spawn_audio_system,
                lifecycle::play_despawn_audio_system,
                world_2d::demo_level::demo_sensor_bridge_system,
                world_2d::demo_level::demo_sensor_audio_system,
                world_2d::demo_level::demo_footstep_audio_system,
            ),
        );
    }
}
