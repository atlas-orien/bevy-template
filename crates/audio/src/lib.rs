pub mod bus;
pub mod playback;
pub mod request;
pub mod source;
pub mod spatial;
pub mod volume;

pub use error::Result;

use bevy::prelude::*;

pub struct AudioFoundationPlugin;

impl Plugin for AudioFoundationPlugin {
    fn build(&self, _app: &mut App) {}
}
