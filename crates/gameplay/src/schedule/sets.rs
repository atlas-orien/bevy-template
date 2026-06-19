use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GameplayUpdateSet {
    ReceiveRuntimeRequests,
    ConsumeRuntimeRequests,
    SyncRuntimeUpdates,
    ApplyLocalInput,
    Movement,
    GameplayRules,
}

pub fn register_schedule_sets(app: &mut App) {
    app.configure_sets(
        Update,
        (
            GameplayUpdateSet::ReceiveRuntimeRequests,
            GameplayUpdateSet::ConsumeRuntimeRequests,
            GameplayUpdateSet::SyncRuntimeUpdates,
            GameplayUpdateSet::ApplyLocalInput,
            GameplayUpdateSet::Movement,
            GameplayUpdateSet::GameplayRules,
        )
            .chain(),
    );
}
