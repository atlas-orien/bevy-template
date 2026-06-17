//! Demo 静物装饰 prefab。

use bevy::prelude::*;
use ecs::components::world::gameplay::GameplaySessionEntityMarker;
use render_2d::products::props::{DemoLandmark2d, DemoRock2d};

use crate::Prefab;

const DEMO_ROCK_Z: f32 = 1.0;
const DEMO_LANDMARK_Z: f32 = -2.0;

pub struct DemoRockPrefab {
    position: Vec2,
}

#[derive(Bundle)]
struct DemoRockBundle {
    session: GameplaySessionEntityMarker,
    visual: DemoRock2d,
}

impl DemoRockBundle {
    fn new(position: Vec2) -> Self {
        Self {
            session: GameplaySessionEntityMarker,
            visual: DemoRock2d::new(Vec3::new(position.x, position.y, DEMO_ROCK_Z)),
        }
    }
}

impl DemoRockPrefab {
    pub fn new(position: Vec2) -> Self {
        Self { position }
    }
}

impl Prefab for DemoRockPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands.spawn(DemoRockBundle::new(self.position)).id()
    }
}

pub struct DemoLandmarkPrefab {
    position: Vec2,
    color: Color,
}

#[derive(Bundle)]
struct DemoLandmarkBundle {
    session: GameplaySessionEntityMarker,
    visual: DemoLandmark2d,
}

impl DemoLandmarkBundle {
    fn new(position: Vec2, color: Color) -> Self {
        Self {
            session: GameplaySessionEntityMarker,
            visual: DemoLandmark2d::new(Vec3::new(position.x, position.y, DEMO_LANDMARK_Z), color),
        }
    }
}

impl DemoLandmarkPrefab {
    pub fn new(position: Vec2, color: Color) -> Self {
        Self { position, color }
    }
}

impl Prefab for DemoLandmarkPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands
            .spawn(DemoLandmarkBundle::new(self.position, self.color))
            .id()
    }
}
