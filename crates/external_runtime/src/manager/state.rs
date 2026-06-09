use std::collections::HashSet;

use prefab::identity::GameplayEntityId;

#[derive(Debug, Default)]
pub struct ManagerState {
    entities: HashSet<GameplayEntityId>,
}

impl ManagerState {
    pub fn register_entity(&mut self, id: GameplayEntityId) {
        self.entities.insert(id);
    }

    pub fn unregister_entity(&mut self, id: GameplayEntityId) {
        self.entities.remove(&id);
    }

    pub fn has_entity(&self, id: GameplayEntityId) -> bool {
        self.entities.contains(&id)
    }

    pub fn entity_ids(&self) -> Vec<GameplayEntityId> {
        self.entities.iter().copied().collect()
    }

    pub fn clear(&mut self) {
        self.entities.clear();
    }
}
