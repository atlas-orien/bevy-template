//! 角色头顶血条覆盖层与血量同步系统。

use bevy::prelude::*;

const DEMO_HEALTH_BAR_WIDTH: f32 = 46.0;
const DEMO_HEALTH_BAR_HEIGHT: f32 = 5.0;
const DEMO_HEALTH_BAR_BACKGROUND_SIZE: Vec2 = Vec2::new(50.0, 9.0);
const DEMO_HEALTH_BAR_TRANSLATION: Vec3 = Vec3::new(0.0, 54.0, 5.0);
const DEMO_HEALTH_BAR_BACKGROUND_COLOR: Color = Color::srgba(0.08, 0.08, 0.08, 0.82);
const DEMO_HEALTH_BAR_FILL_COLOR: Color = Color::srgb(0.18, 0.86, 0.36);
const DEMO_HEALTH_BAR_BACKGROUND_Z: f32 = 0.0;
const DEMO_HEALTH_BAR_FILL_Z: f32 = 0.1;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoHealthBarOverlay2dMarker;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoHealthBarFill2dMarker;

pub struct DemoHealthBarOverlay2d;

#[derive(Bundle)]
struct DemoHealthBarOverlay2dBundle {
    marker: DemoHealthBarOverlay2dMarker,
    transform: Transform,
    visibility: Visibility,
}

impl Default for DemoHealthBarOverlay2dBundle {
    fn default() -> Self {
        Self {
            marker: DemoHealthBarOverlay2dMarker,
            transform: Transform::from_translation(DEMO_HEALTH_BAR_TRANSLATION),
            visibility: Visibility::default(),
        }
    }
}

impl DemoHealthBarOverlay2d {
    pub fn into_bundle(self) -> impl Bundle {
        (
            DemoHealthBarOverlay2dBundle::default(),
            children![
                DemoHealthBarBackground2dBundle::default(),
                DemoHealthBarFill2dBundle::default(),
            ],
        )
    }
}

pub fn set_demo_health_bar_ratio(
    ratio: f32,
    children: &Children,
    fills: &mut Query<(&mut Sprite, &mut Transform), With<DemoHealthBarFill2dMarker>>,
) {
    let ratio = ratio.clamp(0.0, 1.0);
    for child in children {
        let Ok((mut sprite, mut transform)) = fills.get_mut(*child) else {
            continue;
        };
        let width = DEMO_HEALTH_BAR_WIDTH * ratio;
        sprite.custom_size = Some(Vec2::new(width, DEMO_HEALTH_BAR_HEIGHT));
        transform.translation.x = -(DEMO_HEALTH_BAR_WIDTH - width) * 0.5;
    }
}

#[derive(Bundle)]
struct DemoHealthBarBackground2dBundle {
    sprite: Sprite,
    transform: Transform,
}

impl Default for DemoHealthBarBackground2dBundle {
    fn default() -> Self {
        Self {
            sprite: Sprite {
                color: DEMO_HEALTH_BAR_BACKGROUND_COLOR,
                custom_size: Some(DEMO_HEALTH_BAR_BACKGROUND_SIZE),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, DEMO_HEALTH_BAR_BACKGROUND_Z),
        }
    }
}

#[derive(Bundle)]
struct DemoHealthBarFill2dBundle {
    marker: DemoHealthBarFill2dMarker,
    sprite: Sprite,
    transform: Transform,
}

impl Default for DemoHealthBarFill2dBundle {
    fn default() -> Self {
        Self {
            marker: DemoHealthBarFill2dMarker,
            sprite: Sprite {
                color: DEMO_HEALTH_BAR_FILL_COLOR,
                custom_size: Some(Vec2::new(DEMO_HEALTH_BAR_WIDTH, DEMO_HEALTH_BAR_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, DEMO_HEALTH_BAR_FILL_Z),
        }
    }
}
