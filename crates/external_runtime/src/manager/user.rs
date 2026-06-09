use gameplay::api::GameplayRequest;
use gameplay::state::AppState;
use intent::movement::MovementTarget;
use prefab::Prefab;
use prefab::identity::GameplayEntityId;

use super::gameplay::GameplayBridgeApi;

#[derive(Clone)]
pub struct ExternalRuntimeManager {
    gameplay: GameplayBridgeApi,
}

impl ExternalRuntimeManager {
    pub fn new(gameplay: GameplayBridgeApi) -> Self {
        Self { gameplay }
    }

    pub fn spawn_prefab<P>(&self, prefab: P) -> bool
    where
        P: Prefab + Send + Sync + 'static,
    {
        self.gameplay.submit(GameplayRequest::spawn_prefab(prefab))
    }

    pub fn despawn_entity(&self, id: GameplayEntityId) -> bool {
        self.gameplay.submit(GameplayRequest::despawn_entity(id))
    }

    pub fn clear_session(&self) -> bool {
        self.gameplay.submit(GameplayRequest::clear_session())
    }

    pub fn change_state(&self, state: AppState) -> bool {
        self.gameplay.submit(GameplayRequest::change_state(state))
    }

    pub fn set_movement_intent(&self, id: GameplayEntityId, target: MovementTarget) -> bool {
        self.gameplay
            .submit(GameplayRequest::set_movement_intent(id, target))
    }
}
