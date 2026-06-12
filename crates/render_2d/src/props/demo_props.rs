use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoRock2d;

#[derive(Bundle)]
pub struct DemoRock2dBundle {
    pub marker: DemoRock2d,
    pub sprite: Sprite,
    pub transform: Transform,
}

impl DemoRock2dBundle {
    pub fn new(translation: Vec3) -> Self {
        Self {
            marker: DemoRock2d,
            sprite: Sprite {
                color: Color::srgb(0.45, 0.48, 0.52),
                custom_size: Some(Vec2::new(44.0, 30.0)),
                ..default()
            },
            transform: Transform::from_translation(translation),
        }
    }
}

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoSensorZone2d;

#[derive(Bundle)]
pub struct DemoSensorZone2dBundle {
    pub marker: DemoSensorZone2d,
    pub sprite: Sprite,
}

impl Default for DemoSensorZone2dBundle {
    fn default() -> Self {
        Self {
            marker: DemoSensorZone2d,
            sprite: Sprite {
                color: Color::srgba(0.2, 0.75, 0.95, 0.38),
                custom_size: Some(Vec2::new(72.0, 44.0)),
                ..default()
            },
        }
    }
}

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoLandmark2d;

#[derive(Bundle)]
pub struct DemoLandmark2dBundle {
    pub marker: DemoLandmark2d,
    pub sprite: Sprite,
    pub transform: Transform,
}

impl DemoLandmark2dBundle {
    pub fn new(translation: Vec3, color: Color) -> Self {
        Self {
            marker: DemoLandmark2d,
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(28.0, 150.0)),
                ..default()
            },
            transform: Transform::from_translation(translation),
        }
    }
}
