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

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldConfig>();
    }
}
