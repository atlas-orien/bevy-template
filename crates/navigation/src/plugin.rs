use bevy::prelude::*;

use crate::systems::{
    follow_navigation_paths_2d_system, follow_navigation_paths_3d_system,
    refresh_straight_line_paths_2d_system, refresh_straight_line_paths_3d_system,
};

pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                refresh_straight_line_paths_2d_system,
                refresh_straight_line_paths_3d_system,
                follow_navigation_paths_2d_system,
                follow_navigation_paths_3d_system,
            ),
        );
    }
}
