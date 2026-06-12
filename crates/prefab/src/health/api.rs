use bevy::prelude::*;
use ecs::components::base::Health;

pub type HealthQuery<'world, 'state> = Query<'world, 'state, &'static mut Health>;

pub fn damage_health(health: Health, amount: f32) -> Health {
    Health((health.0 - amount.max(0.0)).max(0.0))
}

pub fn damage_entity(
    entity: Entity,
    amount: f32,
    health: &mut HealthQuery<'_, '_>,
) -> Option<Health> {
    let Ok(mut health) = health.get_mut(entity) else {
        return None;
    };

    *health = damage_health(*health, amount);
    Some(*health)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn damage_entity_never_drops_health_below_zero() {
        let health = damage_health(Health(5.0), 8.0);

        assert_eq!(health, Health(0.0));
        assert!(health.is_empty());
    }

    #[test]
    fn damage_entity_ignores_negative_damage() {
        let health = damage_health(Health(5.0), -3.0);

        assert_eq!(health, Health(5.0));
    }
}
