//! Demo 视觉效果：可淡出的纯视觉 sprite。

use bevy::prelude::*;

const DEMO_FLASH_SIZE: Vec2 = Vec2::splat(40.0);
const DEMO_FLASH_Z: f32 = 8.0;
const DEMO_FLASH_SECONDS: f32 = 0.35;
const DEMO_FLASH_COLOR: Color = Color::srgba(1.0, 0.92, 0.42, 0.88);

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub(super) struct DemoEffectLifetime2d {
    remaining_seconds: f32,
    lifetime_seconds: f32,
    initial_alpha: f32,
}

#[derive(Bundle)]
pub struct DemoFlashEffect2d {
    sprite: Sprite,
    transform: Transform,
    lifetime: DemoEffectLifetime2d,
}

impl DemoFlashEffect2d {
    pub fn new(translation: Vec3) -> Self {
        Self {
            sprite: Sprite {
                color: DEMO_FLASH_COLOR,
                custom_size: Some(DEMO_FLASH_SIZE),
                ..default()
            },
            transform: Transform::from_xyz(translation.x, translation.y, DEMO_FLASH_Z),
            lifetime: DemoEffectLifetime2d {
                remaining_seconds: DEMO_FLASH_SECONDS,
                lifetime_seconds: DEMO_FLASH_SECONDS,
                initial_alpha: DEMO_FLASH_COLOR.alpha(),
            },
        }
    }
}

pub(super) fn demo_effect_lifetime_system(
    mut commands: Commands,
    time: Res<Time>,
    mut effects: Query<(Entity, &mut DemoEffectLifetime2d, &mut Sprite)>,
) {
    for (entity, mut lifetime, mut sprite) in &mut effects {
        lifetime.remaining_seconds -= time.delta_secs();
        if lifetime.remaining_seconds <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }

        let life_fraction =
            (lifetime.remaining_seconds / lifetime.lifetime_seconds).clamp(0.0, 1.0);
        sprite
            .color
            .set_alpha(life_fraction * lifetime.initial_alpha);
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    fn app_with_time(delta_seconds: f32) -> App {
        let mut app = App::new();
        let mut time = Time::<()>::default();
        time.advance_by(Duration::from_secs_f32(delta_seconds));
        app.insert_resource(time);
        app
    }

    #[test]
    fn demo_effect_lifetime_despawns_expired_effect() {
        let mut app = app_with_time(DEMO_FLASH_SECONDS);
        app.add_systems(Update, demo_effect_lifetime_system);
        let entity = app
            .world_mut()
            .spawn(DemoFlashEffect2d::new(Vec3::ZERO))
            .id();

        app.update();

        assert!(app.world().get_entity(entity).is_err());
    }

    #[test]
    fn demo_effect_lifetime_fades_sprite_alpha() {
        let mut app = app_with_time(DEMO_FLASH_SECONDS * 0.5);
        app.add_systems(Update, demo_effect_lifetime_system);
        let entity = app
            .world_mut()
            .spawn(DemoFlashEffect2d::new(Vec3::ZERO))
            .id();

        app.update();

        let sprite = app
            .world()
            .get::<Sprite>(entity)
            .expect("sprite should stay");
        assert!(sprite.color.alpha() < DEMO_FLASH_COLOR.alpha());
    }
}
