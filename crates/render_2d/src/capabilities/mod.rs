//! 可复用的 2D 表现能力。

pub mod effects;
pub mod lighting;
pub mod materials;
pub mod mesh;
pub mod particles;
pub mod pixel;
pub mod skeletal_animation;

pub use effects::EffectsPlugin;
pub use particles::ParticlesPlugin;
pub use pixel::PixelPlugin;
pub use skeletal_animation::SkeletalAnimation2dPlugin;

use bevy::prelude::*;

pub struct Render2dCapabilitiesPlugin;

impl Plugin for Render2dCapabilitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SkeletalAnimation2dPlugin,
            EffectsPlugin,
            ParticlesPlugin,
            PixelPlugin,
        ));
    }
}
