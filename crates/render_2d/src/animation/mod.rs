pub mod frame;
pub mod skeletal;

use bevy::prelude::*;

use self::frame::animate_example_frame_2d_system;

pub struct Animation2dPlugin;

impl Plugin for Animation2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_example_frame_2d_system);
    }
}
