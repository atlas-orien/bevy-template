use bevy::prelude::*;
use ecs::systems::lifecycle::despawn_runtime_entities_system;
use ecs::systems::movement::movement_system;

pub struct PrefabRuntimePlugin<S> {
    playing_state: S,
}

impl<S> PrefabRuntimePlugin<S> {
    pub fn new(playing_state: S) -> Self {
        Self { playing_state }
    }
}

impl<S> Plugin for PrefabRuntimePlugin<S>
where
    S: States + Copy,
{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement_system.run_if(in_state(self.playing_state)))
            .add_systems(OnExit(self.playing_state), despawn_runtime_entities_system);
    }
}
