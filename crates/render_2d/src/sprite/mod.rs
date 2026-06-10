pub mod flip;
pub mod systems;

use bevy::prelude::*;

pub use flip::RenderFlip2d;
pub use systems::sync_render_flip_2d_system;

pub struct Sprite2dPlugin;

impl Plugin for Sprite2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sync_render_flip_2d_system);
    }
}
