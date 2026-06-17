use prefab::world_2d::{FollowCamera2dPrefab, UiCamera2dPrefab};

pub struct WorldCamera2d;

pub struct UiCamera2d;

impl WorldCamera2d {
    pub fn prefab() -> FollowCamera2dPrefab {
        FollowCamera2dPrefab
    }
}

impl UiCamera2d {
    pub fn prefab() -> UiCamera2dPrefab {
        UiCamera2dPrefab
    }
}
