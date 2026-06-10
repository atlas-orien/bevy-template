pub mod anchor;
pub mod shape;
pub mod size;
pub mod systems;

use bevy::prelude::*;

pub use anchor::RenderAnchor2d;
pub use shape::RenderShape2d;
pub use size::RenderSize2d;
pub use systems::sync_render_size_2d_system;

pub struct Geometry2dPlugin;

impl Plugin for Geometry2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sync_render_size_2d_system);
    }
}
