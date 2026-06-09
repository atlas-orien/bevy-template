pub mod manager;
pub mod request;
pub mod submit;
pub mod systems;

use bevy::prelude::*;

pub use self::manager::{GameplayManager, GameplayRequestInbox};
pub use self::request::GameplayRequest;
pub use self::submit::submit_gameplay_request;

pub struct GameplayApiPlugin;

impl Plugin for GameplayApiPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<GameplayRequest>();
    }
}
