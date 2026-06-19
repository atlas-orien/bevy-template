//! Object prefab templates and gameplay-facing composition facades.

pub mod control;
pub mod demo_events;
pub mod health;
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
use render_2d::primitives::frame_animation::FrameAnimationSystemSet;

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
                world_2d::demo_level::sync_demo_health_bars_system,
                world_2d::demo_level::spawn_demo_sensor_burst_particles_system,
                world_2d::demo_level::demo_bgm_audio_system,
                world_2d::demo_level::demo_sensor_audio_system,
                world_2d::demo_level::demo_footstep_audio_system,
                navigation::sync_demo_navigation_targets_from_intent_system,
            ),
        )
        .add_systems(
            PostUpdate,
            (
                world_2d::demo_level::sync_demo_frame_animation_from_movement_system,
                world_2d::demo_level::sync_demo_particle_emitters_from_movement_system,
            )
                .before(FrameAnimationSystemSet::Advance),
        );
    }
}
