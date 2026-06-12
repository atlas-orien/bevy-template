use bevy::prelude::*;
use ecs::components::base::{Health, MaxHealth};

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct DemoHealthBarOverlay2d {
    pub width: f32,
}

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoHealthBarFill2d;

#[derive(Bundle)]
pub struct DemoHealthBarOverlay2dBundle {
    pub marker: DemoHealthBarOverlay2d,
    pub transform: Transform,
    pub visibility: Visibility,
}

impl Default for DemoHealthBarOverlay2dBundle {
    fn default() -> Self {
        Self {
            marker: DemoHealthBarOverlay2d { width: 46.0 },
            transform: Transform::from_xyz(0.0, 54.0, 5.0),
            visibility: Visibility::default(),
        }
    }
}

pub fn demo_health_bar_system(
    parents: Query<(&Health, &MaxHealth)>,
    overlays: Query<(&ChildOf, &DemoHealthBarOverlay2d, &Children)>,
    mut fills: Query<(&mut Sprite, &mut Transform), With<DemoHealthBarFill2d>>,
) {
    for (parent, overlay, children) in &overlays {
        let Ok((health, max_health)) = parents.get(parent.parent()) else {
            continue;
        };
        let ratio = if max_health.0 <= 0.0 {
            0.0
        } else {
            (health.0 / max_health.0).clamp(0.0, 1.0)
        };

        for child in children {
            let Ok((mut sprite, mut transform)) = fills.get_mut(*child) else {
                continue;
            };
            let width = overlay.width * ratio;
            sprite.custom_size = Some(Vec2::new(width, 5.0));
            transform.translation.x = -(overlay.width - width) * 0.5;
        }
    }
}

#[derive(Bundle)]
pub struct DemoHealthBarBackground2dBundle {
    pub sprite: Sprite,
    pub transform: Transform,
}

impl Default for DemoHealthBarBackground2dBundle {
    fn default() -> Self {
        Self {
            sprite: Sprite {
                color: Color::srgba(0.08, 0.08, 0.08, 0.82),
                custom_size: Some(Vec2::new(50.0, 9.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
        }
    }
}

#[derive(Bundle)]
pub struct DemoHealthBarFill2dBundle {
    pub marker: DemoHealthBarFill2d,
    pub sprite: Sprite,
    pub transform: Transform,
}

impl Default for DemoHealthBarFill2dBundle {
    fn default() -> Self {
        Self {
            marker: DemoHealthBarFill2d,
            sprite: Sprite {
                color: Color::srgb(0.18, 0.86, 0.36),
                custom_size: Some(Vec2::new(46.0, 5.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.1),
        }
    }
}
