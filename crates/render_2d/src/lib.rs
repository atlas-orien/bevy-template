//! User-editable 2D presentation content, primitives, capabilities, and products.

pub mod capabilities;
pub mod primitives;
pub mod products;

pub use error::Result;

use bevy::prelude::*;

use self::capabilities::Render2dCapabilitiesPlugin;
use self::primitives::Render2dPrimitivesPlugin;
use self::products::Render2dProductsPlugin;

pub struct Render2dPlugin;

impl Plugin for Render2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            Render2dPrimitivesPlugin,
            Render2dCapabilitiesPlugin,
            Render2dProductsPlugin,
        ));
    }
}
