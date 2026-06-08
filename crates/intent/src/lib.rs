pub mod movement;

pub use error::Result;

use bevy::prelude::*;

use self::movement::MovementIntentPlugin;

pub struct IntentPlugin;

impl Plugin for IntentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MovementIntentPlugin);
    }
}
