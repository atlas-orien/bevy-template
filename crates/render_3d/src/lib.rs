//! User-editable 3D presentation primitives, capabilities, and products.

pub mod capabilities;
pub mod primitives;
pub mod products;

pub use error::Result;

use bevy::prelude::*;

use self::capabilities::skeleton::Skeleton3dPlugin;
use self::primitives::Render3dPrimitivesPlugin;

pub struct Render3dPlugin;

impl Plugin for Render3dPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((Render3dPrimitivesPlugin, Skeleton3dPlugin));
    }
}
