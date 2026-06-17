use bevy::prelude::*;
use prefab::world_2d::demo_level::DemoLandmarkPrefab;

pub struct DemoLandmark {
    position: Vec2,
    color: Color,
}

impl DemoLandmark {
    pub fn new(position: Vec2, color: Color) -> Self {
        Self { position, color }
    }

    pub fn prefab(self) -> DemoLandmarkPrefab {
        DemoLandmarkPrefab::new(self.position, self.color)
    }
}
