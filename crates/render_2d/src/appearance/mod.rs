pub mod color;
pub mod opacity;
pub mod systems;
pub mod visibility;

use bevy::prelude::*;

pub use color::RenderColor2d;
pub use opacity::RenderOpacity2d;
pub use systems::{sync_render_color_2d_system, sync_render_visibility_2d_system};
pub use visibility::RenderVisibility2d;

pub struct Appearance2dPlugin;

impl Plugin for Appearance2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                sync_render_color_2d_system,
                sync_render_visibility_2d_system,
            ),
        );
    }
}
