use bevy::prelude::*;

use super::systems::parallax_layer_system;

pub struct Layers2dPlugin;

impl Plugin for Layers2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, parallax_layer_system);
    }
}
