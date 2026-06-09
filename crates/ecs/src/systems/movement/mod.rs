//! 移动规则系统。
//!
//! 这里放根据移动意图、速度、时间和世界规则修改位置或朝向的 ECS 系统函数。

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
