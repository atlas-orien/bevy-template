//! 可复用的 2D 表现能力。

pub mod animation;
pub mod effects;
pub mod lighting;
pub mod materials;
pub mod mesh;
pub mod particles;
pub mod pixel;

pub use animation::CapabilityAnimation2dPlugin;
pub use effects::EffectsPlugin;
pub use lighting::Lighting2dPlugin;
pub use materials::Materials2dPlugin;
pub use mesh::Mesh2dContentPlugin;
pub use particles::ParticlesPlugin;
pub use pixel::PixelPlugin;

use bevy::prelude::*;

pub struct Render2dCapabilitiesPlugin;

impl Plugin for Render2dCapabilitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CapabilityAnimation2dPlugin,
            Lighting2dPlugin,
            Materials2dPlugin,
            Mesh2dContentPlugin,
            EffectsPlugin,
            ParticlesPlugin,
            PixelPlugin,
        ));
    }
}
