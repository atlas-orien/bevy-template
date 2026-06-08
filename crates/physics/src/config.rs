use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Copy)]
pub struct PhysicsConfig {
    pub gravity: Vec2,
    pub debug: bool,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            gravity: Vec2::new(0.0, -980.0),
            debug: false,
        }
    }
}
