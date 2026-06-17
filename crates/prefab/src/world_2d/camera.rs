use bevy::prelude::*;
use ecs::components::world::gameplay::GameplaySessionEntityMarker;
use render_2d::primitives::camera::{FixedCamera2dBundle, FollowCamera2dBundle, UiCamera};

use crate::Prefab;

#[derive(Default)]
pub struct FixedCamera2dPrefab;

#[derive(Default)]
pub struct FollowCamera2dPrefab;

#[derive(Default)]
pub struct UiCamera2dPrefab;

#[derive(Bundle, Default)]
struct FixedCamera2dPrefabBundle {
    camera: FixedCamera2dBundle,
    session: GameplaySessionEntityMarker,
}

#[derive(Bundle, Default)]
struct FollowCamera2dPrefabBundle {
    camera: FollowCamera2dBundle,
    session: GameplaySessionEntityMarker,
}

#[derive(Bundle, Default)]
struct UiCamera2dPrefabBundle {
    camera: UiCamera,
}

impl Prefab for FixedCamera2dPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands.spawn(FixedCamera2dPrefabBundle::default()).id()
    }
}

impl Prefab for FollowCamera2dPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands.spawn(FollowCamera2dPrefabBundle::default()).id()
    }
}

impl Prefab for UiCamera2dPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands.spawn(UiCamera2dPrefabBundle::default()).id()
    }
}
