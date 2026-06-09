//! 生命周期规则系统。
//!
//! 这里放出生、死亡、销毁、重生、状态转换等 ECS 系统函数。

use bevy::prelude::*;

use crate::components::world::gameplay::GameplaySessionEntity;

pub fn despawn_gameplay_entities_system(
    mut commands: Commands,
    entities: Query<Entity, With<GameplaySessionEntity>>,
) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}
