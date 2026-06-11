use bevy::prelude::*;
use render_2d::camera::ui_camera_bundle;

use crate::Prefab;

pub struct UiCameraPrefab;

impl Prefab for UiCameraPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands.spawn(ui_camera_bundle()).id()
    }
}
