pub mod systems;
pub mod z_index;

use bevy::prelude::*;

pub use systems::sync_render_z_index_2d_system;
pub use z_index::RenderZIndex2d;

pub struct Ordering2dPlugin;

impl Plugin for Ordering2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sync_render_z_index_2d_system);
    }
}
