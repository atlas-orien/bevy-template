//! User-editable 3D presentation primitives, capabilities, and products.

pub mod capabilities;
pub mod primitives;
pub mod products;

pub use error::Result;

use bevy::prelude::*;

use self::capabilities::Render3dCapabilitiesPlugin;
use self::primitives::Render3dPrimitivesPlugin;
use self::products::Render3dProductsPlugin;

pub struct Render3dPlugin;

impl Plugin for Render3dPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            Render3dPrimitivesPlugin,
            Render3dCapabilitiesPlugin,
            Render3dProductsPlugin,
        ));
    }
}
