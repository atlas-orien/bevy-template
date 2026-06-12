use bevy::prelude::*;
use render_2d::ui::DemoMenuRoot;

pub struct CleanupPlugin;

impl Plugin for CleanupPlugin {
    fn build(&self, _app: &mut App) {}
}

pub fn despawn_demo_menu_system(
    mut commands: Commands,
    menu_roots: Query<Entity, With<DemoMenuRoot>>,
) {
    for menu_root in &menu_roots {
        commands.entity(menu_root).despawn();
    }
}
