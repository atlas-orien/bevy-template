//! Demo gameplay data 到 2D 表现状态的桥接。

use bevy::prelude::*;
use ecs::components::base::{Facing, Health, MaxHealth, MovementIntent};
use ecs::events::demo_sensor::DemoSensorTriggeredEvent;
use render_2d::capabilities::particles::{
    DemoParticleEmitter2dState, spawn_demo_sensor_particle_burst,
};
use render_2d::primitives::frame_animation::{
    FrameAnimation2d, FrameAnimationFacingFlip2dMarker, FrameAnimationMovementClips2d,
};
use render_2d::products::overlays::{
    DemoHealthBarFill2dMarker, DemoHealthBarOverlay2dMarker, set_demo_health_bar_ratio,
};

pub fn sync_demo_frame_animation_from_movement_system(
    parents: Query<(&MovementIntent, Option<&Facing>)>,
    mut sprites: Query<(
        &ChildOf,
        &FrameAnimationMovementClips2d,
        Option<&FrameAnimationFacingFlip2dMarker>,
        &mut FrameAnimation2d,
        &mut Sprite,
    )>,
) {
    for (parent, clips, facing_flip, mut animation, mut sprite) in &mut sprites {
        let Ok((movement, facing)) = parents.get(parent.parent()) else {
            continue;
        };

        animation.set_clip(if movement.is_moving() {
            clips.moving.as_str()
        } else {
            clips.idle.as_str()
        });

        if facing_flip.is_some()
            && let Some(facing) = facing
        {
            sprite.flip_x = *facing == Facing::Left;
        }
    }
}

pub fn sync_demo_particle_emitters_from_movement_system(
    parents: Query<&MovementIntent>,
    mut emitters: Query<(&ChildOf, &mut DemoParticleEmitter2dState)>,
) {
    for (parent, mut emitter) in &mut emitters {
        let Ok(movement) = parents.get(parent.parent()) else {
            continue;
        };

        emitter.set_enabled(movement.is_moving());
    }
}

pub fn sync_demo_health_bars_system(
    parents: Query<(&Health, &MaxHealth)>,
    overlays: Query<(&ChildOf, &Children), With<DemoHealthBarOverlay2dMarker>>,
    mut fills: Query<(&mut Sprite, &mut Transform), With<DemoHealthBarFill2dMarker>>,
) {
    for (parent, children) in &overlays {
        let Ok((health, max_health)) = parents.get(parent.parent()) else {
            continue;
        };
        let ratio = if max_health.0 <= 0.0 {
            0.0
        } else {
            health.0 / max_health.0
        };

        set_demo_health_bar_ratio(ratio, children, &mut fills);
    }
}

pub fn spawn_demo_sensor_burst_particles_system(
    mut commands: Commands,
    mut events: MessageReader<DemoSensorTriggeredEvent>,
    transforms: Query<&GlobalTransform>,
) {
    for event in events.read() {
        let Ok(transform) = transforms.get(event.sensor) else {
            continue;
        };

        spawn_demo_sensor_particle_burst(&mut commands, transform.translation());
    }
}
