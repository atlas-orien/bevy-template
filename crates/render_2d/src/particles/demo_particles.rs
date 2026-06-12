use bevy::prelude::*;
use ecs::components::base::MovementIntent;
use ecs::events::demo_sensor::DemoSensorTriggeredEvent;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct DemoParticleEmitter2d {
    pub enabled: bool,
    pub particles_per_second: f32,
    pub particle_lifetime_seconds: f32,
    pub emission_accumulator: f32,
    pub max_live_particles: usize,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct DemoParticle2d {
    pub remaining_seconds: f32,
    pub lifetime_seconds: f32,
    pub velocity: Vec2,
}

#[derive(Bundle)]
pub struct DemoParticleEmitter2dBundle {
    pub emitter: DemoParticleEmitter2d,
    pub transform: Transform,
}

impl Default for DemoParticleEmitter2dBundle {
    fn default() -> Self {
        Self {
            emitter: DemoParticleEmitter2d {
                enabled: false,
                particles_per_second: 24.0,
                particle_lifetime_seconds: 0.35,
                emission_accumulator: 0.0,
                max_live_particles: 256,
            },
            transform: Transform::from_xyz(0.0, 2.0, 3.0),
        }
    }
}

pub fn demo_player_dust_system(
    parents: Query<&MovementIntent>,
    mut emitters: Query<(&ChildOf, &mut DemoParticleEmitter2d)>,
) {
    for (parent, mut emitter) in &mut emitters {
        let Ok(movement) = parents.get(parent.parent()) else {
            continue;
        };

        emitter.enabled = movement.is_moving();
    }
}

pub fn demo_particle_emission_system(
    mut commands: Commands,
    time: Res<Time>,
    live_particles: Query<(), With<DemoParticle2d>>,
    mut emitters: Query<(&GlobalTransform, &mut DemoParticleEmitter2d)>,
) {
    let live_count = live_particles.iter().len();

    for (transform, mut emitter) in &mut emitters {
        if !emitter.enabled || live_count >= emitter.max_live_particles {
            emitter.emission_accumulator = 0.0;
            continue;
        }

        emitter.emission_accumulator += emitter.particles_per_second * time.delta_secs();
        while emitter.emission_accumulator >= 1.0 {
            emitter.emission_accumulator -= 1.0;
            let position = transform.translation();
            commands.spawn((
                Sprite {
                    color: Color::srgba(0.88, 0.82, 0.66, 0.72),
                    custom_size: Some(Vec2::splat(6.0)),
                    ..default()
                },
                Transform::from_xyz(position.x, position.y, 3.0),
                DemoParticle2d {
                    remaining_seconds: emitter.particle_lifetime_seconds,
                    lifetime_seconds: emitter.particle_lifetime_seconds,
                    velocity: Vec2::new(-18.0, 18.0),
                },
            ));
        }
    }
}

pub fn demo_particle_update_system(
    mut commands: Commands,
    time: Res<Time>,
    mut particles: Query<(Entity, &mut DemoParticle2d, &mut Transform, &mut Sprite)>,
) {
    for (entity, mut particle, mut transform, mut sprite) in &mut particles {
        particle.remaining_seconds -= time.delta_secs();
        if particle.remaining_seconds <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }

        transform.translation.x += particle.velocity.x * time.delta_secs();
        transform.translation.y += particle.velocity.y * time.delta_secs();

        let alpha = (particle.remaining_seconds / particle.lifetime_seconds).clamp(0.0, 1.0);
        sprite.color.set_alpha(alpha * 0.72);
    }
}

pub fn demo_sensor_particle_burst_system(
    mut commands: Commands,
    mut events: MessageReader<DemoSensorTriggeredEvent>,
    transforms: Query<&GlobalTransform>,
) {
    const DIRECTIONS: [Vec2; 8] = [
        Vec2::new(1.0, 0.0),
        Vec2::new(0.7, 0.7),
        Vec2::new(0.0, 1.0),
        Vec2::new(-0.7, 0.7),
        Vec2::new(-1.0, 0.0),
        Vec2::new(-0.7, -0.7),
        Vec2::new(0.0, -1.0),
        Vec2::new(0.7, -0.7),
    ];

    for event in events.read() {
        let Ok(transform) = transforms.get(event.sensor) else {
            continue;
        };
        let position = transform.translation();

        for direction in DIRECTIONS {
            commands.spawn((
                Sprite {
                    color: Color::srgba(0.28, 0.86, 1.0, 0.86),
                    custom_size: Some(Vec2::splat(8.0)),
                    ..default()
                },
                Transform::from_xyz(position.x, position.y, 4.0),
                DemoParticle2d {
                    remaining_seconds: 0.45,
                    lifetime_seconds: 0.45,
                    velocity: direction.normalize_or_zero() * 90.0,
                },
            ));
        }
    }
}
