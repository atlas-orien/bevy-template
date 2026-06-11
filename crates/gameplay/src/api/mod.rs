pub mod channel;
pub mod runtime_channel;
pub mod submit;
pub mod systems;

use bevy::prelude::*;

pub use self::channel::{
    ManagerUpdateChannel, RuntimeRequestChannel, RuntimeRequestInbox, RuntimeRequestSender,
    RuntimeUpdateInbox, RuntimeUpdateSender, drain_runtime_requests_into,
};
pub use self::runtime_channel::{
    RuntimeEntityRegistrationMessage, RuntimeObjectId, RuntimeRequestMessage, RuntimeSpawnContext,
    RuntimeSpawnRequestMessage, RuntimeUpdateMessage, RuntimeUserId, SpawnItem,
};
pub use self::submit::submit_gameplay_request;

pub struct GameplayApiPlugin;

impl Plugin for GameplayApiPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<RuntimeRequestMessage>();
    }
}
