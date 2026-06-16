pub mod demo_particles;
mod plugin;

pub use demo_particles::{
    DemoParticleEmitter2d, DemoParticleEmitter2dState, spawn_demo_sensor_particle_burst,
};
pub use plugin::ParticlesPlugin;
