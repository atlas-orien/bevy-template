use prefab::world_2d::FollowCamera2dPrefab;

pub struct WorldCamera2d;

impl WorldCamera2d {
    pub fn prefab() -> FollowCamera2dPrefab {
        FollowCamera2dPrefab
    }
}
