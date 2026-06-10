use bevy::prelude::*;

use crate::PhysicsCollisionGroups;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct PhysicsCharacterController2d {
    pub translation: Option<Vec2>,
    pub up: Vec2,
    pub offset: f32,
    pub slide: bool,
    pub max_slope_climb_angle: f32,
    pub min_slope_slide_angle: f32,
    pub snap_to_ground: Option<f32>,
    pub apply_impulse_to_dynamic_bodies: bool,
    pub filter_groups: Option<PhysicsCollisionGroups>,
    pub exclude_sensors: bool,
}

impl Default for PhysicsCharacterController2d {
    fn default() -> Self {
        Self {
            translation: None,
            up: Vec2::Y,
            offset: 0.01,
            slide: true,
            max_slope_climb_angle: std::f32::consts::FRAC_PI_4,
            min_slope_slide_angle: std::f32::consts::FRAC_PI_4,
            snap_to_ground: None,
            apply_impulse_to_dynamic_bodies: true,
            filter_groups: None,
            exclude_sensors: true,
        }
    }
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct PhysicsCharacterControllerOutput2d {
    pub grounded: bool,
    pub desired_translation: Vec2,
    pub effective_translation: Vec2,
    pub is_sliding_down_slope: bool,
    pub collisions: Vec<PhysicsCharacterCollision2d>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PhysicsCharacterCollision2d {
    pub entity: Entity,
    pub character_translation: Vec2,
    pub character_rotation: f32,
    pub translation_applied: Vec2,
    pub translation_remaining: Vec2,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct PhysicsCharacterController3d {
    pub translation: Option<Vec3>,
    pub up: Vec3,
    pub offset: f32,
    pub slide: bool,
    pub max_slope_climb_angle: f32,
    pub min_slope_slide_angle: f32,
    pub snap_to_ground: Option<f32>,
    pub apply_impulse_to_dynamic_bodies: bool,
    pub filter_groups: Option<PhysicsCollisionGroups>,
    pub exclude_sensors: bool,
}

impl Default for PhysicsCharacterController3d {
    fn default() -> Self {
        Self {
            translation: None,
            up: Vec3::Y,
            offset: 0.01,
            slide: true,
            max_slope_climb_angle: std::f32::consts::FRAC_PI_4,
            min_slope_slide_angle: std::f32::consts::FRAC_PI_4,
            snap_to_ground: None,
            apply_impulse_to_dynamic_bodies: true,
            filter_groups: None,
            exclude_sensors: true,
        }
    }
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct PhysicsCharacterControllerOutput3d {
    pub grounded: bool,
    pub desired_translation: Vec3,
    pub effective_translation: Vec3,
    pub is_sliding_down_slope: bool,
    pub collisions: Vec<PhysicsCharacterCollision3d>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PhysicsCharacterCollision3d {
    pub entity: Entity,
    pub character_translation: Vec3,
    pub character_rotation: Quat,
    pub translation_applied: Vec3,
    pub translation_remaining: Vec3,
}
