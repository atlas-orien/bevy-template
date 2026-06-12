pub mod demo_particles;
pub mod example;
mod plugin;

pub use demo_particles::{
    DemoParticle2d, DemoParticleEmitter2d, DemoParticleEmitter2dBundle,
    demo_particle_emission_system, demo_particle_update_system, demo_player_dust_system,
    demo_sensor_particle_burst_system,
};
pub use plugin::ParticlesPlugin;
