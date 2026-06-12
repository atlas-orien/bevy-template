pub mod demo_particles;
pub mod example;

use bevy::prelude::*;

pub use demo_particles::{
    DemoParticle2d, DemoParticleEmitter2d, DemoParticleEmitter2dBundle,
    demo_particle_emission_system, demo_particle_update_system, demo_player_dust_system,
    demo_sensor_particle_burst_system,
};

pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                demo_player_dust_system,
                demo_sensor_particle_burst_system,
                demo_particle_emission_system,
                demo_particle_update_system,
            )
                .chain(),
        );
    }
}
