use bevy::prelude::*;
use ecs::components::base::MovementIntent;
use ecs::events::demo_sensor::DemoSensorTriggeredEvent;

const DEMO_DUST_PARTICLES_PER_SECOND: f32 = 24.0;
const DEMO_DUST_PARTICLE_LIFETIME_SECONDS: f32 = 0.35;
const DEMO_DUST_MAX_LIVE_PARTICLES: usize = 256;
const DEMO_DUST_EMITTER_TRANSLATION: Vec3 = Vec3::new(0.0, 2.0, 3.0);
const DEMO_DUST_PARTICLE_COLOR: Color = Color::srgba(0.88, 0.82, 0.66, 0.72);
const DEMO_DUST_PARTICLE_SIZE: Vec2 = Vec2::splat(6.0);
const DEMO_DUST_PARTICLE_Z: f32 = 3.0;
const DEMO_DUST_PARTICLE_VELOCITY: Vec2 = Vec2::new(-18.0, 18.0);
const DEMO_PARTICLE_EMIT_THRESHOLD: f32 = 1.0;
const DEMO_BURST_PARTICLE_COLOR: Color = Color::srgba(0.28, 0.86, 1.0, 0.86);
const DEMO_BURST_PARTICLE_SIZE: Vec2 = Vec2::splat(8.0);
const DEMO_BURST_PARTICLE_Z: f32 = 4.0;
const DEMO_BURST_PARTICLE_LIFETIME_SECONDS: f32 = 0.45;
const DEMO_BURST_PARTICLE_SPEED: f32 = 90.0;

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
    /// 生成时 sprite 颜色的 alpha；淡出按剩余寿命比例向 0 衰减到它。
    pub initial_alpha: f32,
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
                particles_per_second: DEMO_DUST_PARTICLES_PER_SECOND,
                particle_lifetime_seconds: DEMO_DUST_PARTICLE_LIFETIME_SECONDS,
                emission_accumulator: 0.0,
                max_live_particles: DEMO_DUST_MAX_LIVE_PARTICLES,
            },
            transform: Transform::from_translation(DEMO_DUST_EMITTER_TRANSLATION),
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
        while emitter.emission_accumulator >= DEMO_PARTICLE_EMIT_THRESHOLD {
            emitter.emission_accumulator -= DEMO_PARTICLE_EMIT_THRESHOLD;
            let position = transform.translation();
            commands.spawn((
                Sprite {
                    color: DEMO_DUST_PARTICLE_COLOR,
                    custom_size: Some(DEMO_DUST_PARTICLE_SIZE),
                    ..default()
                },
                Transform::from_xyz(position.x, position.y, DEMO_DUST_PARTICLE_Z),
                DemoParticle2d {
                    remaining_seconds: emitter.particle_lifetime_seconds,
                    lifetime_seconds: emitter.particle_lifetime_seconds,
                    velocity: DEMO_DUST_PARTICLE_VELOCITY,
                    initial_alpha: DEMO_DUST_PARTICLE_COLOR.alpha(),
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

        let life_fraction =
            (particle.remaining_seconds / particle.lifetime_seconds).clamp(0.0, 1.0);
        sprite
            .color
            .set_alpha(life_fraction * particle.initial_alpha);
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
                    color: DEMO_BURST_PARTICLE_COLOR,
                    custom_size: Some(DEMO_BURST_PARTICLE_SIZE),
                    ..default()
                },
                Transform::from_xyz(position.x, position.y, DEMO_BURST_PARTICLE_Z),
                DemoParticle2d {
                    remaining_seconds: DEMO_BURST_PARTICLE_LIFETIME_SECONDS,
                    lifetime_seconds: DEMO_BURST_PARTICLE_LIFETIME_SECONDS,
                    velocity: direction.normalize_or_zero() * DEMO_BURST_PARTICLE_SPEED,
                    initial_alpha: DEMO_BURST_PARTICLE_COLOR.alpha(),
                },
            ));
        }
    }
}
