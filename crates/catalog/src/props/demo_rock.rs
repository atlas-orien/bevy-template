use bevy::prelude::*;
use prefab::world_2d::demo_level::DemoRockPrefab;

pub struct DemoRock {
    position: Vec2,
}

impl DemoRock {
    pub fn at(position: Vec2) -> Self {
        Self { position }
    }

    pub fn prefab(self) -> DemoRockPrefab {
        DemoRockPrefab::new(self.position)
    }
}
