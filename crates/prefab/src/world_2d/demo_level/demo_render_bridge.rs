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
    hierarchy: Query<&ChildOf>,
    movement_entities: Query<(&MovementIntent, Option<&Facing>)>,
    mut sprites: Query<(
        &ChildOf,
        &FrameAnimationMovementClips2d,
        Option<&FrameAnimationFacingFlip2dMarker>,
        &mut FrameAnimation2d,
        &mut Sprite,
    )>,
) {
    for (parent, clips, facing_flip, mut animation, mut sprite) in &mut sprites {
        let Some((movement, facing)) =
            find_parent_movement_entity(parent.parent(), &hierarchy, &movement_entities)
        else {
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
    hierarchy: Query<&ChildOf>,
    movement_entities: Query<(&MovementIntent, Option<&Facing>)>,
    mut emitters: Query<(&ChildOf, &mut DemoParticleEmitter2dState)>,
) {
    for (parent, mut emitter) in &mut emitters {
        let Some((movement, _)) =
            find_parent_movement_entity(parent.parent(), &hierarchy, &movement_entities)
        else {
            continue;
        };

        emitter.set_enabled(movement.is_moving());
    }
}

pub fn sync_demo_health_bars_system(
    hierarchy: Query<&ChildOf>,
    health_entities: Query<(&Health, &MaxHealth)>,
    overlays: Query<(&ChildOf, &Children), With<DemoHealthBarOverlay2dMarker>>,
    mut fills: Query<(&mut Sprite, &mut Transform), With<DemoHealthBarFill2dMarker>>,
) {
    for (parent, children) in &overlays {
        let Some((health, max_health)) =
            find_parent_health_entity(parent.parent(), &hierarchy, &health_entities)
        else {
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

fn find_parent_movement_entity<'world, 'state>(
    mut entity: Entity,
    hierarchy: &Query<'world, 'state, &ChildOf>,
    movement_entities: &'world Query<'world, 'state, (&MovementIntent, Option<&Facing>)>,
) -> Option<(&'world MovementIntent, Option<&'world Facing>)> {
    loop {
        if let Ok(movement) = movement_entities.get(entity) {
            return Some(movement);
        }

        entity = hierarchy.get(entity).ok()?.parent();
    }
}

fn find_parent_health_entity<'world, 'state>(
    mut entity: Entity,
    hierarchy: &Query<'world, 'state, &ChildOf>,
    health_entities: &'world Query<'world, 'state, (&Health, &MaxHealth)>,
) -> Option<(&'world Health, &'world MaxHealth)> {
    loop {
        if let Ok(health) = health_entities.get(entity) {
            return Some(health);
        }

        entity = hierarchy.get(entity).ok()?.parent();
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

#[cfg(test)]
mod tests {
    use ecs::components::base::{MovementTarget, Speed};

    use super::*;

    #[derive(Bundle)]
    struct TestMovingPlayerBundle {
        movement: MovementIntent,
        speed: Speed,
        facing: Facing,
    }

    #[derive(Bundle)]
    struct TestAnimatedSpriteBundle {
        parent: ChildOf,
        movement_clips: FrameAnimationMovementClips2d,
        facing_flip: FrameAnimationFacingFlip2dMarker,
        animation: FrameAnimation2d,
        sprite: Sprite,
    }

    #[derive(Bundle)]
    struct TestVisualRootBundle {
        parent: ChildOf,
        transform: Transform,
        visibility: Visibility,
    }

    #[test]
    fn movement_sync_selects_walk_clip_and_flips_left_facing_sprite() {
        let mut app = App::new();
        app.add_systems(Update, sync_demo_frame_animation_from_movement_system);

        let player = app
            .world_mut()
            .spawn(TestMovingPlayerBundle {
                movement: MovementIntent {
                    target: MovementTarget::Direction(Vec2::NEG_X),
                },
                speed: Speed::default(),
                facing: Facing::Left,
            })
            .id();

        let sprite = app
            .world_mut()
            .spawn(TestAnimatedSpriteBundle {
                parent: ChildOf(player),
                movement_clips: FrameAnimationMovementClips2d::new("idle", "walk"),
                facing_flip: FrameAnimationFacingFlip2dMarker,
                animation: FrameAnimation2d::new("idle"),
                sprite: Sprite::default(),
            })
            .id();

        app.update();

        let animation = app.world().get::<FrameAnimation2d>(sprite).unwrap();
        let sprite = app.world().get::<Sprite>(sprite).unwrap();
        assert_eq!(animation.clip, "walk");
        assert!(sprite.flip_x);
    }

    #[test]
    fn movement_sync_finds_gameplay_entity_above_visual_root() {
        let mut app = App::new();
        app.add_systems(Update, sync_demo_frame_animation_from_movement_system);

        let player = app
            .world_mut()
            .spawn(TestMovingPlayerBundle {
                movement: MovementIntent {
                    target: MovementTarget::Direction(Vec2::NEG_X),
                },
                speed: Speed::default(),
                facing: Facing::Left,
            })
            .id();
        let visual_root = app
            .world_mut()
            .spawn(TestVisualRootBundle {
                parent: ChildOf(player),
                transform: Transform::default(),
                visibility: Visibility::default(),
            })
            .id();
        let sprite = app
            .world_mut()
            .spawn(TestAnimatedSpriteBundle {
                parent: ChildOf(visual_root),
                movement_clips: FrameAnimationMovementClips2d::new("idle", "walk"),
                facing_flip: FrameAnimationFacingFlip2dMarker,
                animation: FrameAnimation2d::new("idle"),
                sprite: Sprite::default(),
            })
            .id();

        app.update();

        let animation = app.world().get::<FrameAnimation2d>(sprite).unwrap();
        let sprite = app.world().get::<Sprite>(sprite).unwrap();
        assert_eq!(animation.clip, "walk");
        assert!(sprite.flip_x);
    }
}
