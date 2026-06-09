pub mod channel;
pub mod request;
pub mod submit;
pub mod systems;

use bevy::prelude::*;

pub use self::channel::{
    GameplayRequestInbox, GameplayRequestSender, GameplayUpdate, GameplayUpdateInbox,
    GameplayUpdateSender, gameplay_channels,
};
pub use self::request::GameplayRequest;
pub use self::submit::submit_gameplay_request;

pub struct GameplayApiPlugin;

impl Plugin for GameplayApiPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<GameplayRequest>();
    }
}
