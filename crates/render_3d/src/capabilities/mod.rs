//! 可复用的 3D 表现能力。

pub mod animation;
pub mod effects;
pub mod particles;

pub use animation::Animation3dPlugin;
pub use effects::Effects3dPlugin;
pub use particles::Particles3dPlugin;

use bevy::prelude::*;

pub struct Render3dCapabilitiesPlugin;

impl Plugin for Render3dCapabilitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((Animation3dPlugin, Effects3dPlugin, Particles3dPlugin));
    }
}
