use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct ExampleSpecialMaterial2d;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct ExampleMaterialTint2d {
    pub color: Color,
    pub intensity: f32,
}

#[derive(Bundle)]
pub struct ExampleMaterialTaggedSprite2dBundle {
    pub marker: ExampleSpecialMaterial2d,
    pub tint: ExampleMaterialTint2d,
    pub sprite: Sprite,
}

impl ExampleMaterialTaggedSprite2dBundle {
    pub fn new(color: Color, intensity: f32, size: Vec2) -> Self {
        Self {
            marker: ExampleSpecialMaterial2d,
            tint: ExampleMaterialTint2d { color, intensity },
            sprite: Sprite {
                color,
                custom_size: Some(size),
                ..default()
            },
        }
    }
}
