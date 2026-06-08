//! 移动规则系统。
//!
//! 这里放根据移动意图、速度、时间和世界规则修改位置或朝向的 ECS 系统函数。

use bevy::prelude::*;

use crate::components::characters::player::{MovementIntent, PlayerSpeed};

pub fn movement_system(
    time: Res<Time>,
    mut movers: Query<(&MovementIntent, &PlayerSpeed, &mut Transform)>,
) {
    for (movement_intent, speed, mut transform) in &mut movers {
        let velocity = movement_intent.direction * speed.0 * time.delta_secs();
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}
