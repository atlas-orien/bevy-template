use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct ExampleParticleEmitter2d;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct ExampleParticleEmission2d {
    pub particles_per_second: f32,
    pub particle_lifetime_seconds: f32,
    pub initial_velocity: Vec2,
}

#[derive(Bundle)]
pub struct ExampleParticleEmitter2dBundle {
    pub marker: ExampleParticleEmitter2d,
    pub emission: ExampleParticleEmission2d,
    pub transform: Transform,
}

impl ExampleParticleEmitter2dBundle {
    pub fn new(
        translation: Vec3,
        particles_per_second: f32,
        particle_lifetime_seconds: f32,
        initial_velocity: Vec2,
    ) -> Self {
        Self {
            marker: ExampleParticleEmitter2d,
            emission: ExampleParticleEmission2d {
                particles_per_second,
                particle_lifetime_seconds,
                initial_velocity,
            },
            transform: Transform::from_translation(translation),
        }
    }
}
