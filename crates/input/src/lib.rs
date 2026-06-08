mod local;

pub use error::Result;

use bevy::prelude::*;

use self::local::LocalInputPlugin;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LocalInputPlugin);
    }
}
