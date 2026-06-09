use prefab::Prefab;

use crate::api::SpawnItem;

#[derive(Default)]
pub struct GameplaySpawnPlan {
    prefabs: Vec<Box<dyn SpawnItem>>,
}

impl GameplaySpawnPlan {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with<P>(mut self, prefab: P) -> Self
    where
        P: Prefab + Send + Sync + 'static,
    {
        self.prefabs.push(Box::new(prefab));
        self
    }

    pub fn into_prefabs(self) -> Vec<Box<dyn SpawnItem>> {
        self.prefabs
    }
}
