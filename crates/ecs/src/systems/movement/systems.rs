use bevy::prelude::*;

use crate::components::base::{Facing, MovementIntent, MovementTarget, Speed};

const POSITION_TARGET_EPSILON: f32 = 1.0;

pub fn movement_system(
    time: Res<Time>,
    mut movers: Query<(
        &mut MovementIntent,
        &Speed,
        &mut Transform,
        Option<&mut Facing>,
    )>,
) {
    for (mut movement_intent, speed, mut transform, facing) in &mut movers {
        let direction = match movement_intent.target {
            MovementTarget::None => Vec2::ZERO,
            MovementTarget::Direction(direction) => direction.normalize_or_zero(),
            MovementTarget::Position(target) => {
                let current = transform.translation.truncate();
                let offset = target - current;

                if offset.length() <= POSITION_TARGET_EPSILON {
                    movement_intent.target = MovementTarget::None;
                    Vec2::ZERO
                } else {
                    offset.normalize_or_zero()
                }
            }
        };

        let velocity = direction * speed.0 * time.delta_secs();
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;

        if let Some(mut facing) = facing {
            if direction.x < 0.0 {
                *facing = Facing::Left;
            } else if direction.x > 0.0 {
                *facing = Facing::Right;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    const TEST_SPEED: f32 = 60.0;
    const ONE_FRAME_DELTA: f32 = 1.0 / 60.0;

    fn movement_app() -> App {
        let mut app = App::new();
        let mut time = Time::<()>::default();
        time.advance_by(Duration::from_secs_f32(ONE_FRAME_DELTA));
        app.insert_resource(time)
            .add_systems(Update, movement_system);
        app
    }

    #[test]
    fn direction_movement_uses_speed_and_delta() {
        let mut app = movement_app();
        let entity = app
            .world_mut()
            .spawn((
                MovementIntent {
                    target: MovementTarget::Direction(Vec2::X),
                },
                Speed(TEST_SPEED),
                Transform::default(),
            ))
            .id();

        app.update();

        let transform = app.world().get::<Transform>(entity).unwrap();
        assert!((transform.translation.x - TEST_SPEED * ONE_FRAME_DELTA).abs() < f32::EPSILON);
        assert_eq!(transform.translation.y, 0.0);
    }

    #[test]
    fn diagonal_direction_is_normalized() {
        let mut app = movement_app();
        let entity = app
            .world_mut()
            .spawn((
                MovementIntent {
                    target: MovementTarget::Direction(Vec2::ONE),
                },
                Speed(TEST_SPEED),
                Transform::default(),
            ))
            .id();

        app.update();

        let transform = app.world().get::<Transform>(entity).unwrap();
        let distance = transform.translation.truncate().length();
        assert!((distance - TEST_SPEED * ONE_FRAME_DELTA).abs() < f32::EPSILON);
    }

    #[test]
    fn position_target_inside_epsilon_stops_movement() {
        let mut app = movement_app();
        let entity = app
            .world_mut()
            .spawn((
                MovementIntent {
                    target: MovementTarget::Position(Vec2::new(0.5, 0.0)),
                },
                Speed(TEST_SPEED),
                Transform::default(),
            ))
            .id();

        app.update();

        let movement = app.world().get::<MovementIntent>(entity).unwrap();
        assert!(matches!(movement.target, MovementTarget::None));
    }

    #[test]
    fn horizontal_movement_updates_facing() {
        let mut app = movement_app();
        let entity = app
            .world_mut()
            .spawn((
                MovementIntent {
                    target: MovementTarget::Direction(Vec2::NEG_X),
                },
                Speed(TEST_SPEED),
                Transform::default(),
                Facing::Right,
            ))
            .id();

        app.update();

        assert_eq!(*app.world().get::<Facing>(entity).unwrap(), Facing::Left);
    }

    #[test]
    fn vertical_movement_keeps_facing() {
        let mut app = movement_app();
        let entity = app
            .world_mut()
            .spawn((
                MovementIntent {
                    target: MovementTarget::Direction(Vec2::Y),
                },
                Speed(TEST_SPEED),
                Transform::default(),
                Facing::Left,
            ))
            .id();

        app.update();

        assert_eq!(*app.world().get::<Facing>(entity).unwrap(), Facing::Left);
    }
}
