//! Demo 粒子：脚下扬尘发射与感应区爆发的纯视觉生命周期。

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
pub(super) struct DemoParticleEmitter2dMarker {
    enabled: bool,
    particles_per_second: f32,
    particle_lifetime_seconds: f32,
    emission_accumulator: f32,
    max_live_particles: usize,
}

impl DemoParticleEmitter2dMarker {
    #[cfg(test)]
    fn emission_accumulator(&self) -> f32 {
        self.emission_accumulator
    }

    fn reset_accumulator(&mut self) {
        self.emission_accumulator = 0.0;
    }

    fn accumulate(&mut self, delta_seconds: f32) -> usize {
        self.emission_accumulator += self.particles_per_second * delta_seconds;

        let mut emit_count = 0;
        while self.emission_accumulator >= DEMO_PARTICLE_EMIT_THRESHOLD {
            self.emission_accumulator -= DEMO_PARTICLE_EMIT_THRESHOLD;
            emit_count += 1;
        }

        emit_count
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub(super) struct DemoParticle2d {
    remaining_seconds: f32,
    lifetime_seconds: f32,
    velocity: Vec2,
    /// 生成时 sprite 颜色的 alpha；淡出按剩余寿命比例向 0 衰减到它。
    initial_alpha: f32,
}

#[derive(Bundle)]
pub struct DemoParticleEmitter2d {
    emitter: DemoParticleEmitter2dMarker,
    transform: Transform,
}

impl Default for DemoParticleEmitter2d {
    fn default() -> Self {
        Self {
            emitter: DemoParticleEmitter2dMarker {
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

pub(super) fn demo_player_dust_system(
    parents: Query<&MovementIntent>,
    mut emitters: Query<(&ChildOf, &mut DemoParticleEmitter2dMarker)>,
) {
    for (parent, mut emitter) in &mut emitters {
        let Ok(movement) = parents.get(parent.parent()) else {
            continue;
        };

        emitter.enabled = movement.is_moving();
    }
}

pub(super) fn demo_particle_emission_system(
    mut commands: Commands,
    time: Res<Time>,
    live_particles: Query<(), With<DemoParticle2d>>,
    mut emitters: Query<(&GlobalTransform, &mut DemoParticleEmitter2dMarker)>,
) {
    let live_count = live_particles.iter().len();

    for (transform, mut emitter) in &mut emitters {
        if !emitter.enabled || live_count >= emitter.max_live_particles {
            emitter.reset_accumulator();
            continue;
        }

        let emit_count = emitter.accumulate(time.delta_secs());
        for _ in 0..emit_count {
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

pub(super) fn demo_particle_update_system(
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

pub(super) fn demo_sensor_particle_burst_system(
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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    const TEST_LIFETIME_SECONDS: f32 = 2.0;
    const TEST_ALPHA: f32 = 0.8;

    fn app_with_time(delta_seconds: f32) -> App {
        let mut app = App::new();
        let mut time = Time::<()>::default();
        time.advance_by(Duration::from_secs_f32(delta_seconds));
        app.insert_resource(time);
        app
    }

    #[test]
    fn particle_lifetime_despawns_expired_particles() {
        let mut app = app_with_time(TEST_LIFETIME_SECONDS);
        app.add_systems(Update, demo_particle_update_system);
        let entity = app
            .world_mut()
            .spawn((
                DemoParticle2d {
                    remaining_seconds: 0.1,
                    lifetime_seconds: TEST_LIFETIME_SECONDS,
                    velocity: Vec2::ZERO,
                    initial_alpha: TEST_ALPHA,
                },
                Transform::default(),
                Sprite::default(),
            ))
            .id();

        app.update();

        assert!(app.world().get_entity(entity).is_err());
    }

    #[test]
    fn particle_alpha_fades_by_remaining_lifetime() {
        let mut app = app_with_time(TEST_LIFETIME_SECONDS / 2.0);
        app.add_systems(Update, demo_particle_update_system);
        let entity = app
            .world_mut()
            .spawn((
                DemoParticle2d {
                    remaining_seconds: TEST_LIFETIME_SECONDS,
                    lifetime_seconds: TEST_LIFETIME_SECONDS,
                    velocity: Vec2::ZERO,
                    initial_alpha: TEST_ALPHA,
                },
                Transform::default(),
                Sprite {
                    color: Color::srgba(1.0, 1.0, 1.0, TEST_ALPHA),
                    ..default()
                },
            ))
            .id();

        app.update();

        let sprite = app.world().get::<Sprite>(entity).unwrap();
        assert!((sprite.color.alpha() - TEST_ALPHA * 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn emitter_accumulates_particle_rate() {
        let mut emitter = DemoParticleEmitter2dMarker {
            enabled: true,
            particles_per_second: 4.0,
            particle_lifetime_seconds: TEST_LIFETIME_SECONDS,
            emission_accumulator: 0.0,
            max_live_particles: 10,
        };

        assert_eq!(emitter.accumulate(0.25), 1);
        assert_eq!(emitter.emission_accumulator(), 0.0);
        assert_eq!(emitter.accumulate(0.125), 0);
        assert_eq!(emitter.accumulate(0.125), 1);
    }

    #[test]
    fn emission_system_resets_when_live_particles_reach_limit() {
        let mut app = app_with_time(1.0);
        app.add_systems(Update, demo_particle_emission_system);
        let emitter = app
            .world_mut()
            .spawn((
                DemoParticleEmitter2dMarker {
                    enabled: true,
                    particles_per_second: 4.0,
                    particle_lifetime_seconds: TEST_LIFETIME_SECONDS,
                    emission_accumulator: 0.75,
                    max_live_particles: 1,
                },
                Transform::default(),
                GlobalTransform::default(),
            ))
            .id();
        app.world_mut().spawn((
            DemoParticle2d {
                remaining_seconds: TEST_LIFETIME_SECONDS,
                lifetime_seconds: TEST_LIFETIME_SECONDS,
                velocity: Vec2::ZERO,
                initial_alpha: TEST_ALPHA,
            },
            Sprite::default(),
            Transform::default(),
        ));

        app.update();

        let emitter = app
            .world()
            .get::<DemoParticleEmitter2dMarker>(emitter)
            .unwrap();
        assert_eq!(emitter.emission_accumulator(), 0.0);
    }
}
