use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct WorldConfig {
    pub width: u32,
    pub height: u32,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            width: 1280,
            height: 720,
        }
    }
}
