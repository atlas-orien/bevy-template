use bevy::prelude::*;

use super::session::GameSession;
use super::world::WorldConfig;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldConfig>()
            .init_resource::<GameSession>();
    }
}
