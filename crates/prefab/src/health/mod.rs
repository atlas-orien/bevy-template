use bevy::prelude::*;
use ecs::components::base::Health;

pub type HealthQuery<'world, 'state> = Query<'world, 'state, &'static mut Health>;

pub fn damage_entity(
    entity: Entity,
    amount: f32,
    health: &mut HealthQuery<'_, '_>,
) -> Option<Health> {
    let Ok(mut health) = health.get_mut(entity) else {
        return None;
    };

    health.0 = (health.0 - amount.max(0.0)).max(0.0);
    Some(*health)
}
