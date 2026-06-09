use prefab::world_2d::characters::player::Player2dPrefab;

use super::plan::GameplaySpawnPlan;

pub fn default_gameplay_spawn_plan() -> GameplaySpawnPlan {
    GameplaySpawnPlan::new().with(Player2dPrefab::default())
}
