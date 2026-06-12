use bevy::prelude::*;

use super::{LocalInputContextMessage, LocalUserInputMessage, RuntimeRequestMessage};

pub struct GameplayApiPlugin;

impl Plugin for GameplayApiPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<RuntimeRequestMessage>()
            .add_message::<LocalUserInputMessage>()
            .add_message::<LocalInputContextMessage>();
    }
}
